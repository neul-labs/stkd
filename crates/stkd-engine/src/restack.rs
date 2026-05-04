//! Restack operation - rebase branches onto their parents

use anyhow::Result;
use serde::Serialize;
use stkd_core::{rebase, Repository};

/// Options for the restack operation.
#[derive(Debug, Default, Clone)]
pub struct RestackOptions {
    /// Only restack current branch and descendants
    pub current_only: bool,
    /// Force restack even if branches appear up-to-date
    pub force: bool,
    /// Show what would be done without actually doing it
    pub dry_run: bool,
}

/// Result of a restack operation.
#[derive(Debug, Default, Serialize)]
pub struct RestackResult {
    pub restacked: Vec<RestackEntry>,
}

#[derive(Debug, Serialize)]
pub struct RestackEntry {
    pub branch: String,
    pub onto: String,
    pub status: RestackStatus,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum RestackStatus {
    Success,
    UpToDate,
    Conflict,
    Error,
}

/// Determine which branches need restacking.
pub fn branches_to_restack(
    repo: &Repository,
    opts: &RestackOptions,
) -> Result<Vec<(String, String)>> {
    let graph = repo.load_graph()?;

    let all_branches: Vec<String> = if opts.current_only {
        let current = repo.current_branch()?.ok_or_else(|| {
            anyhow::anyhow!("Not on a branch")
        })?;

        let mut to_restack = vec![current.clone()];
        to_restack.extend(
            graph.descendants(&current).iter().map(|s| s.to_string())
        );
        to_restack
    } else {
        graph.topological_order().iter().map(|s| s.to_string()).collect()
    };

    let branches: Vec<String> = if opts.force {
        all_branches
    } else {
        let needs_restack = graph.needs_restack(repo.git())?;
        all_branches
            .into_iter()
            .filter(|b| needs_restack.contains(b))
            .collect()
    };

    let mut result = Vec::new();
    for branch in &branches {
        if let Some(info) = repo.storage().load_branch(branch)? {
            let onto = if graph.is_trunk(&info.parent) {
                repo.trunk().to_string()
            } else {
                info.parent.clone()
            };
            result.push((branch.clone(), onto));
        }
    }

    Ok(result)
}

/// Restack branches onto their parents.
pub fn restack(repo: &Repository, opts: RestackOptions) -> Result<RestackResult> {
    let mut result = RestackResult::default();

    let branches = branches_to_restack(repo, &opts)?;

    if branches.is_empty() {
        return Ok(result);
    }

    if opts.dry_run {
        for (branch, onto) in &branches {
            result.restacked.push(RestackEntry {
                branch: branch.clone(),
                onto: onto.clone(),
                status: RestackStatus::Success,
            });
        }
        return Ok(result);
    }

    repo.ensure_clean()?;

    for (branch, onto) in &branches {
        match rebase::rebase_branch(repo.git(), branch, onto) {
            Ok(rebase::RebaseResult::Success { .. }) => {
                result.restacked.push(RestackEntry {
                    branch: branch.clone(),
                    onto: onto.clone(),
                    status: RestackStatus::Success,
                });
            }
            Ok(rebase::RebaseResult::UpToDate { .. }) => {
                result.restacked.push(RestackEntry {
                    branch: branch.clone(),
                    onto: onto.clone(),
                    status: RestackStatus::UpToDate,
                });
            }
            Ok(rebase::RebaseResult::Conflict { branch, onto }) => {
                result.restacked.push(RestackEntry {
                    branch,
                    onto,
                    status: RestackStatus::Conflict,
                });
            }
            Err(_) => {
                result.restacked.push(RestackEntry {
                    branch: branch.clone(),
                    onto: onto.clone(),
                    status: RestackStatus::Error,
                });
            }
        }
    }

    Ok(result)
}
