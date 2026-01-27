//! Submit command - push branches and create/update merge requests

use anyhow::{Context, Result};
use clap::Args;
use stkd_core::Repository;
use stkd_provider_api::{CreateMergeRequest, UpdateMergeRequest};
use std::fs;

use crate::output;
use crate::provider_context::ProviderContext;

#[derive(Args)]
pub struct SubmitArgs {
    /// Submit entire stack (current + descendants)
    #[arg(long, short)]
    stack: bool,

    /// Create MRs as draft
    #[arg(long)]
    draft: bool,

    /// Just push, don't create/update MRs
    #[arg(long)]
    push_only: bool,

    /// Don't push, only create/update MRs
    #[arg(long)]
    no_push: bool,

    /// Update MR titles and descriptions
    #[arg(long)]
    update: bool,

    /// Custom MR title (only for single branch)
    #[arg(long, short)]
    title: Option<String>,

    /// Custom MR body (only for single branch)
    #[arg(long, short)]
    body: Option<String>,

    /// Request reviewers (comma-separated usernames)
    #[arg(long, short = 'r', value_delimiter = ',')]
    reviewers: Vec<String>,

    /// Add labels (comma-separated)
    #[arg(long, short = 'l', value_delimiter = ',')]
    labels: Vec<String>,

    /// Use PR template from .github/PULL_REQUEST_TEMPLATE.md
    #[arg(long)]
    template: bool,

    /// Only submit specific branches (comma-separated)
    #[arg(long, value_delimiter = ',')]
    only: Vec<String>,

    /// Submit from this branch to tip
    #[arg(long)]
    from: Option<String>,

    /// Submit from root to this branch
    #[arg(long)]
    to: Option<String>,

    /// Show what would be done without actually doing it
    #[arg(long)]
    dry_run: bool,
}

pub async fn execute(args: SubmitArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    // Get current branch
    let current = repo.current_branch()?.ok_or_else(|| {
        anyhow::anyhow!("Not on a branch")
    })?;

    // Check if current branch is tracked
    if !repo.storage().is_tracked(&current) {
        anyhow::bail!(
            "Branch '{}' is not tracked. Run 'gt track' first.",
            current
        );
    }

    let graph = repo.load_graph()?;

    // Get branches to submit based on options
    let branches: Vec<String> = if !args.only.is_empty() {
        // Specific branches only
        args.only.clone()
    } else if let Some(ref from_branch) = args.from {
        // From specific branch to tip
        let mut to_submit = vec![from_branch.clone()];
        to_submit.extend(
            graph.descendants(from_branch).iter().map(|s| s.to_string())
        );
        to_submit
    } else if let Some(ref to_branch) = args.to {
        // From root to specific branch
        graph.ancestors(to_branch).iter()
            .filter(|b| !graph.is_trunk(b))
            .map(|s| s.to_string())
            .chain(std::iter::once(to_branch.clone()))
            .collect()
    } else if args.stack {
        let mut to_submit = vec![current.clone()];
        to_submit.extend(
            graph.descendants(&current).iter().map(|s| s.to_string())
        );
        to_submit
    } else {
        vec![current.clone()]
    };

    if branches.is_empty() {
        output::warn("No branches to submit");
        return Ok(());
    }

    // Validate custom title/body only for single branch
    if branches.len() > 1 && (args.title.is_some() || args.body.is_some()) {
        anyhow::bail!("--title and --body can only be used when submitting a single branch");
    }

    // Dry run mode
    if args.dry_run {
        output::info("Dry run - showing what would be done:");
        output::info("");

        for branch in &branches {
            let info = repo.storage().load_branch(branch)?;

            let base = if let Some(ref info) = info {
                if graph.is_trunk(&info.parent) {
                    repo.trunk().to_string()
                } else {
                    info.parent.clone()
                }
            } else {
                repo.trunk().to_string()
            };

            let has_mr = info.as_ref().and_then(|i| i.merge_request_id).is_some();
            let mr_action = if has_mr {
                if args.update { "Update MR" } else { "Skip (MR exists)" }
            } else {
                "Create MR"
            };

            if !args.no_push {
                output::info(&format!("  {} Push {} to origin", output::ARROW, branch));
            }
            if !args.push_only {
                output::info(&format!("  {} {} for {} -> {}", output::ARROW, mr_action, branch, base));
                if !args.reviewers.is_empty() {
                    output::info(&format!("       Reviewers: {}", args.reviewers.join(", ")));
                }
                if !args.labels.is_empty() {
                    output::info(&format!("       Labels: {}", args.labels.join(", ")));
                }
            }
        }

        output::info("");
        output::hint("Run without --dry-run to execute");
        return Ok(());
    }

    // Push branches first (unless --no-push)
    if !args.no_push {
        let pb = output::progress_bar(branches.len() as u64, "Pushing branches");

        for branch in &branches {
            pb.set_message(format!("Pushing {}", branch));

            let result = std::process::Command::new("git")
                .args(["push", "-u", "origin", branch, "--force-with-lease"])
                .output()
                .context("Failed to run git push")?;

            if !result.status.success() {
                output::finish_progress_error(&pb, &format!("Failed to push {}", branch));
                anyhow::bail!("Push failed for branch '{}'", branch);
            }

            pb.inc(1);
        }

        output::finish_progress(&pb, &format!("Pushed {} branch(es)", branches.len()));
    }

    // If push-only, we're done
    if args.push_only {
        output::success("Push complete");
        return Ok(());
    }

    // Create provider context (auto-detects GitHub/GitLab from remote)
    let ctx = ProviderContext::from_repo(&repo).await?;

    output::info(&format!("Repository: {} ({})", ctx.full_name(), ctx.provider_type));

    // Load PR template if requested
    let template_body = if args.template {
        load_pr_template(&repo)
    } else {
        None
    };

    // Build stack info for MR bodies
    let stack_branches: Vec<(String, Option<u64>)> = {
        let all_stack = graph.stack(&current);
        all_stack.iter()
            .map(|b| {
                let mr_num = repo.storage()
                    .load_branch(b)
                    .ok()
                    .flatten()
                    .and_then(|info| info.merge_request_id);
                (b.to_string(), mr_num)
            })
            .collect()
    };

    // Create/update MRs
    for branch in &branches {
        let info = repo.storage()
            .load_branch(branch)?
            .context("Branch info not found")?;

        // Determine base branch (parent)
        let base = if graph.is_trunk(&info.parent) {
            repo.trunk().to_string()
        } else {
            info.parent.clone()
        };

        // Check if MR already exists
        if let Some(mr_number) = info.merge_request_id {
            // Update existing MR
            if args.update {
                output::info(&format!("  {} Updating MR #{} for {}...", output::ARROW, mr_number, branch));

                // Generate new body with stack info
                let custom_body = args.body.as_deref()
                    .or(template_body.as_deref());
                let new_body = generate_stack_body(&stack_branches, branch, custom_body);

                let update = UpdateMergeRequest {
                    title: args.title.clone(),
                    body: Some(new_body),
                    target_branch: Some(base),
                    labels: if args.labels.is_empty() { None } else { Some(args.labels.clone()) },
                    ..Default::default()
                };

                ctx.provider().update_mr(&ctx.repo_id, mr_number.into(), update)
                    .await
                    .context("Failed to update MR")?;

                output::success(&format!("Updated MR #{} for {}", mr_number, branch));

                if !args.reviewers.is_empty() {
                    output::hint("Note: Reviewers can only be set when creating new MRs");
                }
            } else {
                output::info(&format!("  {} MR #{} exists for {} (use --update to modify)", output::ARROW, mr_number, branch));
            }
        } else {
            // Create new MR
            output::info(&format!("  {} Creating MR for {}...", output::ARROW, branch));

            // Generate title from branch name if not provided
            let title = args.title.clone().unwrap_or_else(|| {
                // Convert branch name to title
                // e.g., "feature/add-login" -> "Add login"
                let name = branch.rsplit('/').next().unwrap_or(branch);
                let title = name.replace('-', " ").replace('_', " ");
                // Capitalize first letter
                let mut chars = title.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            });

            // Generate body with stack info and optional template
            let custom_body = args.body.as_deref()
                .or(template_body.as_deref());
            let body = generate_stack_body(&stack_branches, branch, custom_body);

            let create = CreateMergeRequest {
                title: title.clone(),
                source_branch: branch.clone(),
                target_branch: base,
                body: Some(body),
                draft: args.draft,
                labels: args.labels.clone(),
                reviewers: args.reviewers.clone(),
                ..Default::default()
            };

            let mr = ctx.provider().create_mr(&ctx.repo_id, create)
                .await
                .context("Failed to create MR")?;

            // Save MR number to branch info
            repo.storage().update_branch(branch, |b| {
                b.merge_request_id = Some(mr.number);
                b.merge_request_url = Some(mr.web_url.clone());
            })?;

            let draft_indicator = if args.draft { " (draft)" } else { "" };
            output::success(&format!("Created MR #{}{} for {}", mr.number, draft_indicator, branch));
            output::info(&format!("     {}", mr.web_url));

            if !args.reviewers.is_empty() {
                output::info(&format!("     Reviewers: {}", args.reviewers.join(", ")));
            }
            if !args.labels.is_empty() {
                output::info(&format!("     Labels: {}", args.labels.join(", ")));
            }
        }
    }

    // Update all MRs in the stack to reflect current stack state
    if args.stack && !args.update {
        output::info("Updating stack visualization in all MRs...");

        // Refresh stack info with new MR numbers
        let updated_stack: Vec<(String, Option<u64>)> = {
            let all_stack = graph.stack(&current);
            all_stack.iter()
                .map(|b| {
                    let mr_num = repo.storage()
                        .load_branch(b)
                        .ok()
                        .flatten()
                        .and_then(|info| info.merge_request_id);
                    (b.to_string(), mr_num)
                })
                .collect()
        };

        for (branch_name, mr_number) in &updated_stack {
            if let Some(mr_num) = mr_number {
                let body = generate_stack_body(&updated_stack, branch_name, None);
                let update = UpdateMergeRequest {
                    body: Some(body),
                    ..Default::default()
                };

                if let Err(e) = ctx.provider().update_mr(&ctx.repo_id, (*mr_num).into(), update).await {
                    output::warn(&format!("Failed to update stack info in MR #{}: {}", mr_num, e));
                }
            }
        }
    }

    output::success("Submit complete");
    output::hint("Run 'gt log' to see MR status");

    Ok(())
}

/// Load PR template from common locations
fn load_pr_template(repo: &Repository) -> Option<String> {
    let workdir = repo.git().path().parent()?;

    // Check common template locations
    let template_paths = [
        ".github/PULL_REQUEST_TEMPLATE.md",
        ".github/pull_request_template.md",
        ".github/PULL_REQUEST_TEMPLATE/default.md",
        "docs/pull_request_template.md",
        ".gitlab/merge_request_templates/Default.md",
    ];

    for template_path in &template_paths {
        let full_path = workdir.join(template_path);
        if full_path.exists() {
            if let Ok(content) = fs::read_to_string(&full_path) {
                output::info(&format!("Using PR template from {}", template_path));
                return Some(content);
            }
        }
    }

    None
}

/// Generate the body for a merge request with stack visualization.
fn generate_stack_body(
    stack: &[(String, Option<u64>)],
    current_branch: &str,
    custom_body: Option<&str>,
) -> String {
    let mut body = String::new();

    // Add custom body if provided
    if let Some(custom) = custom_body {
        body.push_str(custom);
        body.push_str("\n\n");
    }

    // Add stack visualization
    body.push_str("---\n\n");
    body.push_str("## Stack\n\n");

    for (branch, mr_num) in stack.iter().rev() {
        let is_current = branch == current_branch;
        let prefix = if is_current { "**" } else { "" };
        let suffix = if is_current { "** (this MR)" } else { "" };

        if let Some(num) = mr_num {
            body.push_str(&format!("- {}#{}{}\n", prefix, num, suffix));
        } else {
            body.push_str(&format!("- {}`{}`{}\n", prefix, branch, suffix));
        }
    }

    body.push_str("\n---\n");
    body.push_str("*Managed by [Stack](https://github.com/neul-labs/stack)*\n");

    body
}
