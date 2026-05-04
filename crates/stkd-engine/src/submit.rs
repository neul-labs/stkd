//! Submit operation - push branches and create/update merge requests

use anyhow::{Context, Result};
use serde::Serialize;
use stkd_core::{Repository, Storage};
use stkd_core::dag::BranchGraph;
use stkd_provider_api::{CreateMergeRequest, Provider, RepoId, UpdateMergeRequest};

use crate::retry::{with_retry, DEFAULT_MAX_RETRIES};

/// Options for the submit operation.
#[derive(Debug, Default, Clone)]
pub struct SubmitOptions {
    /// Submit entire stack (current + descendants)
    pub stack: bool,
    /// Create MRs as draft
    pub draft: bool,
    /// Just push, don't create/update MRs
    pub push_only: bool,
    /// Don't push, only create/update MRs
    pub no_push: bool,
    /// Update MR titles and descriptions
    pub update: bool,
    /// Custom MR title (only for single branch)
    pub title: Option<String>,
    /// Custom MR body (only for single branch)
    pub body: Option<String>,
    /// Request reviewers (comma-separated usernames)
    pub reviewers: Vec<String>,
    /// Add labels (comma-separated)
    pub labels: Vec<String>,
    /// Use PR template from .github/PULL_REQUEST_TEMPLATE.md
    pub template: bool,
    /// Only submit specific branches (comma-separated)
    pub only: Vec<String>,
    /// Submit from this branch to tip
    pub from: Option<String>,
    /// Submit from root to this branch
    pub to: Option<String>,
    /// Show what would be done without actually doing it
    pub dry_run: bool,
}

/// Result of a submit operation.
#[derive(Debug, Default, Serialize)]
pub struct SubmitResult {
    pub pushed_branches: Vec<String>,
    pub created: Vec<CreatedMr>,
    pub updated: Vec<UpdatedMr>,
    pub skipped: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CreatedMr {
    pub branch: String,
    pub number: u64,
    pub url: String,
    pub draft: bool,
}

#[derive(Debug, Serialize)]
pub struct UpdatedMr {
    pub branch: String,
    pub number: u64,
}

/// Push branches to remote.
fn push_branches(repo: &Repository, branches: &[String]) -> Result<Vec<String>> {
    let mut pushed = Vec::new();
    for branch in branches {
        let result = std::process::Command::new("git")
            .args(["push", "-u", "origin", branch, "--force-with-lease"])
            .output()
            .context("Failed to run git push")?;

        if !result.status.success() {
            anyhow::bail!("Push failed for branch '{}'", branch);
        }
        pushed.push(branch.clone());
    }
    Ok(pushed)
}

/// Load PR template from common locations.
pub fn load_pr_template(repo: &Repository) -> Option<String> {
    let workdir = repo.git().path().parent()?;

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
            if let Ok(content) = std::fs::read_to_string(&full_path) {
                return Some(content);
            }
        }
    }

    None
}

/// Generate the body for a merge request with stack visualization.
pub fn generate_stack_body(
    stack: &[(String, Option<u64>)],
    current_branch: &str,
    custom_body: Option<&str>,
) -> String {
    let mut body = String::new();

    if let Some(custom) = custom_body {
        body.push_str(custom);
        body.push_str("\n\n");
    }

    body.push_str("---\n\n");
    body.push_str("## Stack\n\n");

    for (branch, mr_num) in stack.iter().rev() {
        let is_current = branch == current_branch;
        let prefix = if is_current { "**" } else { "" };
        let suffix = if is_current { "** (this MR)" } else { "" };

        if let Some(num) = mr_num {
            body.push_str(&format!("- {}{}{}\n", prefix, num, suffix));
        } else {
            body.push_str(&format!("- {}`{}`{}\n", prefix, branch, suffix));
        }
    }

    body.push_str("\n---\n");
    body.push_str("*Managed by [Stack](https://github.com/neul-labs/stkd)*\n");

    body
}

fn build_stack_branches(repo: &Repository, graph: &BranchGraph, current: &str) -> Result<Vec<(String, Option<u64>)>> {
    let all_stack = graph.stack(current);
    let mut result = Vec::new();
    for b in &all_stack {
        let mr_num = repo.storage()
            .load_branch(b)
            .ok()
            .flatten()
            .and_then(|info| info.merge_request_id);
        result.push((b.to_string(), mr_num));
    }
    Ok(result)
}

/// Determine which branches to submit based on options.
pub fn select_branches(
    repo: &Repository,
    graph: &BranchGraph,
    current: &str,
    opts: &SubmitOptions,
) -> Result<Vec<String>> {
    let branches: Vec<String> = if !opts.only.is_empty() {
        opts.only.clone()
    } else if let Some(ref from_branch) = opts.from {
        let mut to_submit = vec![from_branch.clone()];
        to_submit.extend(
            graph.descendants(from_branch).iter().map(|s| s.to_string())
        );
        to_submit
    } else if let Some(ref to_branch) = opts.to {
        graph.ancestors(to_branch).iter()
            .filter(|b| !graph.is_trunk(b))
            .map(|s| s.to_string())
            .chain(std::iter::once(to_branch.clone()))
            .collect()
    } else if opts.stack {
        let mut to_submit = vec![current.to_string()];
        to_submit.extend(
            graph.descendants(current).iter().map(|s| s.to_string())
        );
        to_submit
    } else {
        vec![current.to_string()]
    };

    Ok(branches)
}

/// Generate a default title from a branch name.
pub fn default_title(branch: &str) -> String {
    let name = branch.rsplit('/').next().unwrap_or(branch);
    let title = name.replace('-', " ").replace('_', " ");
    let mut chars = title.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}

/// Submit branches - push and create/update MRs.
pub async fn submit(
    repo: &Repository,
    opts: SubmitOptions,
    provider: &dyn Provider,
    repo_id: &RepoId,
) -> Result<SubmitResult> {
    let current = repo.current_branch()?.ok_or_else(|| {
        anyhow::anyhow!("Not on a branch")
    })?;

    if !repo.storage().is_tracked(&current) {
        anyhow::bail!("Branch '{}' is not tracked. Run 'gt track' first.", current);
    }

    let graph = repo.load_graph()?;
    let branches = select_branches(repo, &graph, &current, &opts)?;

    if branches.is_empty() {
        return Ok(SubmitResult::default());
    }

    if branches.len() > 1 && (opts.title.is_some() || opts.body.is_some()) {
        anyhow::bail!("--title and --body can only be used when submitting a single branch");
    }

    let mut result = SubmitResult::default();

    // Dry run - just return empty result
    if opts.dry_run {
        return Ok(result);
    }

    // Push branches first (unless --no-push)
    if !opts.no_push {
        result.pushed_branches = push_branches(repo, &branches)?;
    }

    // If push-only, we're done
    if opts.push_only {
        return Ok(result);
    }

    // Load PR template if requested
    let template_body = if opts.template {
        load_pr_template(repo)
    } else {
        None
    };

    // Build stack info for MR bodies
    let stack_branches = build_stack_branches(repo, &graph, &current)?;

    // Create/update MRs
    for branch in &branches {
        let info = repo.storage()
            .load_branch(branch)?
            .context("Branch info not found")?;

        let base = if graph.is_trunk(&info.parent) {
            repo.trunk().to_string()
        } else {
            info.parent.clone()
        };

        if let Some(mr_number) = info.merge_request_id {
            // Update existing MR
            if opts.update {
                let custom_body = opts.body.as_deref().or(template_body.as_deref());
                let new_body = generate_stack_body(&stack_branches,
                    branch,
                    custom_body,
                );

                let update = UpdateMergeRequest {
                    title: opts.title.clone(),
                    body: Some(new_body),
                    target_branch: Some(base),
                    labels: if opts.labels.is_empty() { None } else { Some(opts.labels.clone()) },
                    ..Default::default()
                };

                with_retry(|| provider.update_mr(repo_id, mr_number.into(), update.clone()), DEFAULT_MAX_RETRIES)
                    .await
                    .context("Failed to update MR")?;

                result.updated.push(UpdatedMr {
                    branch: branch.clone(),
                    number: mr_number,
                });
            } else {
                result.skipped.push(branch.clone());
            }
        } else {
            // Create new MR
            let title = opts.title.clone().unwrap_or_else(|| default_title(branch));

            let custom_body = opts.body.as_deref()
                .or(template_body.as_deref());
            let body = generate_stack_body(&stack_branches,
                branch,
                custom_body,
            );

            let create = CreateMergeRequest {
                title: title.clone(),
                source_branch: branch.clone(),
                target_branch: base,
                body: Some(body),
                draft: opts.draft,
                labels: opts.labels.clone(),
                reviewers: opts.reviewers.clone(),
                ..Default::default()
            };

            let mr = with_retry(|| provider.create_mr(repo_id, create.clone()), DEFAULT_MAX_RETRIES)
                .await
                .context("Failed to create MR")?;

            // Save MR number to branch info
            repo.storage().update_branch(branch, |b| {
                b.merge_request_id = Some(mr.number);
                b.merge_request_url = Some(mr.web_url.clone());
            })?;

            result.created.push(CreatedMr {
                branch: branch.clone(),
                number: mr.number,
                url: mr.web_url,
                draft: opts.draft,
            });
        }
    }

    // Update all MRs in the stack to reflect current stack state
    if opts.stack && !opts.update {
        let updated_stack = build_stack_branches(repo, &graph, &current)?;

        for (branch_name, mr_number) in &updated_stack {
            if let Some(mr_num) = mr_number {
                let body = generate_stack_body(&updated_stack,
                    branch_name,
                    None,
                );
                let update = UpdateMergeRequest {
                    body: Some(body),
                    ..Default::default()
                };

                let _ = with_retry(|| provider.update_mr(repo_id, (*mr_num).into(), update.clone()), DEFAULT_MAX_RETRIES).await;
            }
        }
    }

    Ok(result)
}
