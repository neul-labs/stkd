//! Land operation - merge MRs and clean up branches

use anyhow::{Context, Result};
use serde::Serialize;
use stkd_core::Repository;
use stkd_provider_api::{MergeMethod, Provider, RepoId};

use crate::retry::{with_retry, DEFAULT_MAX_RETRIES};

/// Options for the land operation.
#[derive(Debug, Default, Clone)]
pub struct LandOptions {
    /// Merge method (merge, squash, rebase, ff)
    pub method: String,
    /// Land the entire stack from bottom to top
    pub stack: bool,
    /// Delete local branches after landing
    pub delete_local: bool,
    /// Don't sync after landing
    pub no_sync: bool,
    /// Show what would be done without actually doing it
    pub dry_run: bool,
}

/// Result of a land operation.
#[derive(Debug, Default, Serialize)]
pub struct LandResult {
    pub landed: Vec<LandedBranch>,
    pub deleted: Vec<String>,
    pub current_switched_to_trunk: bool,
}

#[derive(Debug, Serialize)]
pub struct LandedBranch {
    pub branch: String,
    pub mr_number: u64,
    pub merged: bool,
    pub message: String,
}

/// Parse merge method string.
pub fn parse_merge_method(method: &str) -> Result<MergeMethod> {
    match method.to_lowercase().as_str() {
        "merge" => Ok(MergeMethod::Merge),
        "squash" => Ok(MergeMethod::Squash),
        "rebase" => Ok(MergeMethod::Rebase),
        "ff" | "fast-forward" => Ok(MergeMethod::FastForward),
        _ => anyhow::bail!(
            "Invalid merge method: {}. Use 'merge', 'squash', 'rebase', or 'ff'.",
            method
        ),
    }
}

/// Land branches - merge MRs and clean up.
pub async fn land(
    repo: &Repository,
    opts: LandOptions,
    provider: &dyn Provider,
    repo_id: &RepoId,
) -> Result<LandResult> {
    let mut result = LandResult::default();

    let current = repo
        .current_branch()?
        .ok_or_else(|| anyhow::anyhow!("Not on a branch"))?;

    if !repo.storage().is_tracked(&current) {
        anyhow::bail!("Branch '{}' is not tracked. Run 'gt track' first.", current);
    }

    let graph = repo.load_graph()?;

    let info = repo
        .storage()
        .load_branch(&current)?
        .context("Branch info not found")?;

    let mr_number = info.merge_request_id.ok_or_else(|| {
        anyhow::anyhow!(
            "No MR found for branch '{}'. Run 'gt submit' first.",
            current
        )
    })?;

    let merge_method = parse_merge_method(&opts.method)?;

    let branches_to_land: Vec<(String, u64)> = if opts.stack {
        let mut to_land = Vec::new();
        let ancestors = graph.ancestors(&current);

        for ancestor in ancestors.iter().rev() {
            if let Some(info) = repo.storage().load_branch(ancestor)? {
                if let Some(mr_num) = info.merge_request_id {
                    to_land.push((ancestor.to_string(), mr_num));
                }
            }
        }

        to_land.push((current.clone(), mr_number));
        to_land
    } else {
        vec![(current.clone(), mr_number)]
    };

    if opts.dry_run {
        for (branch, mr_num) in &branches_to_land {
            result.landed.push(LandedBranch {
                branch: branch.clone(),
                mr_number: *mr_num,
                merged: false,
                message: "dry run".to_string(),
            });
        }
        return Ok(result);
    }

    for (branch, mr_num) in &branches_to_land {
        match with_retry(
            || provider.merge_mr(repo_id, (*mr_num).into(), merge_method),
            DEFAULT_MAX_RETRIES,
        )
        .await
        {
            Ok(merge_result) => {
                result.landed.push(LandedBranch {
                    branch: branch.clone(),
                    mr_number: *mr_num,
                    merged: merge_result.merged,
                    message: merge_result.message,
                });
            }
            Err(e) => {
                anyhow::bail!("Failed to merge MR #{}: {}", mr_num, e);
            }
        }

        // Delete local branch if requested
        if opts.delete_local && *branch != current {
            if let Err(e) = repo.delete_branch(branch, true) {
                tracing::warn!("Failed to delete local branch {}: {}", branch, e);
            } else {
                result.deleted.push(branch.clone());
            }
        }
    }

    // Sync with remote to update state
    if !opts.no_sync {
        let _ = std::process::Command::new("git")
            .args(["fetch", "origin"])
            .status()
            .context("Failed to run git fetch");

        // Checkout trunk if we landed the current branch
        if branches_to_land.iter().any(|(b, _)| b == &current) {
            repo.checkout(repo.trunk())?;
            result.current_switched_to_trunk = true;

            let _ = std::process::Command::new("git")
                .args(["pull", "--ff-only"])
                .status();

            if opts.delete_local {
                let _ = std::process::Command::new("git")
                    .args(["branch", "-D", &current])
                    .status();

                let _ = repo.untrack_branch(&current);
                result.deleted.push(current.clone());
            }
        }
    }

    Ok(result)
}
