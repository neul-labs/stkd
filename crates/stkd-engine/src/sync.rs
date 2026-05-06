//! Sync operation - sync with remote and update stack state

use anyhow::{Context, Result};
use serde::Serialize;
use stkd_core::{rebase, Repository};
use stkd_provider_api::{Provider, RepoId};

use crate::retry::{with_retry, DEFAULT_MAX_RETRIES};

/// Options for the sync operation.
#[derive(Debug, Default, Clone)]
pub struct SyncOptions {
    /// Don't delete merged branches
    pub no_delete: bool,
    /// Don't restack after sync
    pub no_restack: bool,
    /// Don't update trunk
    pub no_pull: bool,
    /// Force restack even if not needed
    pub force: bool,
    /// Show what would be done without actually doing it
    pub dry_run: bool,
}

/// Result of a sync operation.
#[derive(Debug, Default, Serialize)]
pub struct SyncResult {
    pub fetched: bool,
    pub trunk_updated: bool,
    pub merged_branches: Vec<String>,
    pub closed_branches: Vec<String>,
    pub deleted_branches: Vec<String>,
    pub restacked: Vec<RestackedBranch>,
    pub conflicts: Vec<Conflict>,
    pub current_branch: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RestackedBranch {
    pub branch: String,
    pub up_to_date: bool,
}

#[derive(Debug, Serialize)]
pub struct Conflict {
    pub branch: String,
    pub onto: String,
}

/// Sync repository with remote.
pub async fn sync(
    repo: &Repository,
    opts: SyncOptions,
    provider: Option<&dyn Provider>,
    repo_id: Option<&RepoId>,
) -> Result<SyncResult> {
    let mut result = SyncResult::default();
    let trunk = repo.trunk().to_string();
    let current_branch = repo.current_branch()?;
    result.current_branch = current_branch.clone();

    if opts.dry_run {
        return Ok(result);
    }

    // Step 1: Fetch from remote
    let fetch_result = std::process::Command::new("git")
        .args(["fetch", "origin", "--prune"])
        .output()
        .context("Failed to run git fetch")?;

    result.fetched = fetch_result.status.success();

    // Step 2: Update trunk
    if !opts.no_pull && result.fetched {
        repo.checkout(&trunk)?;

        let status = std::process::Command::new("git")
            .args(["pull", "--ff-only", "origin", &trunk])
            .status()
            .context("Failed to run git pull")?;

        result.trunk_updated = status.success();

        // Return to original branch if it still exists
        if let Some(ref branch) = current_branch {
            if branch != &trunk
                && repo
                    .git()
                    .find_branch(branch, git2::BranchType::Local)
                    .is_ok()
            {
                repo.checkout(branch)?;
            }
        }
    }

    // Step 3: Update MR status from provider
    if let (Some(provider), Some(repo_id)) = (provider, repo_id) {
        let branches = repo.storage().list_branches()?;

        for branch_info in branches {
            if let Some(mr_number) = branch_info.merge_request_id {
                if let Ok(mr) = with_retry(
                    || provider.get_mr(repo_id, mr_number.into()),
                    DEFAULT_MAX_RETRIES,
                )
                .await
                {
                    match mr.state {
                        stkd_provider_api::MergeRequestState::Merged => {
                            result.merged_branches.push(branch_info.name.clone());
                        }
                        stkd_provider_api::MergeRequestState::Closed => {
                            let remote_ref = format!("refs/remotes/origin/{}", branch_info.name);
                            if repo.git().find_reference(&remote_ref).is_err() {
                                result.merged_branches.push(branch_info.name.clone());
                            } else {
                                result.closed_branches.push(branch_info.name.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Step 4: Clean up merged branches
    if !opts.no_delete && !result.merged_branches.is_empty() {
        for branch in &result.merged_branches {
            let _ = std::process::Command::new("git")
                .args(["branch", "-D", branch])
                .status();

            let _ = repo.untrack_branch(branch);
            result.deleted_branches.push(branch.clone());
        }
    }

    // Step 5: Restack branches
    if !opts.no_restack {
        let graph = repo.load_graph()?;
        let needs_restack = graph.needs_restack(repo.git())?;

        if !needs_restack.is_empty() || opts.force {
            match rebase::restack_all(repo.git(), repo.storage(), &graph) {
                Ok(results) => {
                    for r in results {
                        match r {
                            rebase::RebaseResult::Success { branch, .. } => {
                                result.restacked.push(RestackedBranch {
                                    branch,
                                    up_to_date: false,
                                });
                            }
                            rebase::RebaseResult::UpToDate { branch } => {
                                result.restacked.push(RestackedBranch {
                                    branch,
                                    up_to_date: true,
                                });
                            }
                            rebase::RebaseResult::Conflict { branch, onto } => {
                                result.conflicts.push(Conflict { branch, onto });
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    // Return to original branch or trunk if merged
    if let Some(ref branch) = current_branch {
        if !result.merged_branches.contains(branch) {
            if repo
                .git()
                .find_branch(branch, git2::BranchType::Local)
                .is_ok()
            {
                repo.checkout(branch)?;
            }
        } else {
            repo.checkout(&trunk)?;
        }
    }

    Ok(result)
}
