//! Submit command - push branches and create/update merge requests

use anyhow::{Context, Result};
use clap::Args;
use stkd_core::Repository;

use crate::output;

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

pub async fn execute(args: SubmitArgs, json: bool) -> Result<()> {
    let repo = Repository::open(".")?;
    let current = repo
        .current_branch()?
        .ok_or_else(|| anyhow::anyhow!("Not on a branch"))?;

    if !repo.storage().is_tracked(&current) {
        anyhow::bail!("Branch '{}' is not tracked. Run 'gt track' first.", current);
    }

    let opts = stkd_engine::SubmitOptions {
        stack: args.stack,
        draft: args.draft,
        push_only: args.push_only,
        no_push: args.no_push,
        update: args.update,
        title: args.title,
        body: args.body,
        reviewers: args.reviewers,
        labels: args.labels,
        template: args.template,
        only: args.only,
        from: args.from,
        to: args.to,
        dry_run: args.dry_run,
    };

    let graph = repo.load_graph()?;
    let branches = stkd_engine::select_branches(&repo, &graph, &current, &opts)?;

    if branches.is_empty() {
        if json {
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &serde_json::json!({"skipped": true, "reason": "No branches to submit" })
                )?
            );
        } else {
            output::warn("No branches to submit");
        }
        return Ok(());
    }

    if branches.len() > 1 && (opts.title.is_some() || opts.body.is_some()) {
        anyhow::bail!("--title and --body can only be used when submitting a single branch");
    }

    if args.dry_run {
        if json {
            let dry_run_info: Vec<serde_json::Value> = branches.iter().map(|branch| {
                let info = repo.storage().load_branch(branch).ok().flatten();
                let base = info.as_ref().map(|i| {
                    if graph.is_trunk(&i.parent) { repo.trunk().to_string() } else { i.parent.clone() }
                }).unwrap_or_else(|| repo.trunk().to_string());
                let has_mr = info.as_ref().and_then(|i| i.merge_request_id).is_some();
                serde_json::json!({
                    "branch": branch,
                    "base": base,
                    "has_mr": has_mr,
                    "action": if has_mr { if args.update { "update" } else { "skip" } } else { "create" }
                })
            }).collect();
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &serde_json::json!({"dry_run": true, "branches": dry_run_info })
                )?
            );
            return Ok(());
        }

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
                if args.update {
                    "Update MR"
                } else {
                    "Skip (MR exists)"
                }
            } else {
                "Create MR"
            };

            if !args.no_push {
                output::info(&format!("  {} Push {} to origin", output::ARROW, branch));
            }
            if !args.push_only {
                output::info(&format!(
                    "  {} {} for {} -> {}",
                    output::ARROW,
                    mr_action,
                    branch,
                    base
                ));
                if !opts.reviewers.is_empty() {
                    output::info(&format!("       Reviewers: {}", opts.reviewers.join(", ")));
                }
                if !opts.labels.is_empty() {
                    output::info(&format!("       Labels: {}", opts.labels.join(", ")));
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
        if json {
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &serde_json::json!({"pushed": branches, "success": true })
                )?
            );
        } else {
            output::success("Push complete");
        }
        return Ok(());
    }

    // Create provider context
    let ctx = stkd_engine::ProviderContext::from_repo(&repo).await?;

    if !json {
        output::info(&format!(
            "Repository: {} ({})",
            ctx.full_name(),
            ctx.provider_type
        ));
    }

    let result = stkd_engine::submit(&repo, opts, ctx.provider(), &ctx.repo_id).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        for created in &result.created {
            let draft_indicator = if created.draft { " (draft)" } else { "" };
            output::success(&format!(
                "Created MR #{}{} for {}",
                created.number, draft_indicator, created.branch
            ));
            output::info(&format!("     {}", created.url));
        }
        for updated in &result.updated {
            output::success(&format!(
                "Updated MR #{} for {}",
                updated.number, updated.branch
            ));
        }
        for skipped in &result.skipped {
            output::info(&format!(
                "  {} MR exists for {} (use --update to modify)",
                output::ARROW,
                skipped
            ));
        }
        output::success("Submit complete");
        output::hint("Run 'gt log' to see MR status");
    }

    Ok(())
}
