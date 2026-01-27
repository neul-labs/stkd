//! Rebase and restack operations
//!
//! Handles rebasing branches onto their updated parents.

use git2::{BranchType, Repository as GitRepo};
use tracing::{debug, info, warn};

use crate::dag::BranchGraph;
use crate::storage::{ConflictState, Storage};
use crate::{Error, Result};

/// Result of a rebase operation
#[derive(Debug)]
pub enum RebaseResult {
    /// Rebase completed successfully
    Success {
        branch: String,
        old_head: String,
        new_head: String,
    },
    /// Rebase has conflicts that need resolution
    Conflict {
        branch: String,
        onto: String,
    },
    /// Nothing to rebase (already up to date)
    UpToDate {
        branch: String,
    },
}

/// Rebase a single branch onto its parent
pub fn rebase_branch(
    repo: &GitRepo,
    branch_name: &str,
    onto: &str,
) -> Result<RebaseResult> {
    info!("Rebasing {} onto {}", branch_name, onto);

    // Get branch references
    let branch = repo.find_branch(branch_name, BranchType::Local)?;
    let onto_branch = repo.find_branch(onto, BranchType::Local)?;

    let branch_commit = branch.get().peel_to_commit()?;
    let onto_commit = onto_branch.get().peel_to_commit()?;

    let old_head = branch_commit.id().to_string();

    // Find merge base
    let merge_base = repo.merge_base(branch_commit.id(), onto_commit.id())?;

    // Check if already up to date
    if merge_base == onto_commit.id() {
        debug!("{} is already based on {}", branch_name, onto);
        return Ok(RebaseResult::UpToDate {
            branch: branch_name.to_string(),
        });
    }

    // Checkout the branch
    repo.set_head(&format!("refs/heads/{}", branch_name))?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;

    // Set up rebase
    let annotated_onto = repo.find_annotated_commit(onto_commit.id())?;
    let annotated_branch = repo.find_annotated_commit(branch_commit.id())?;
    let annotated_base = repo.find_annotated_commit(merge_base)?;

    let mut rebase = repo.rebase(
        Some(&annotated_branch),
        Some(&annotated_onto),
        Some(&annotated_base),
        None,
    )?;

    // Apply each commit
    while let Some(op) = rebase.next() {
        match op {
            Ok(_operation) => {
                // Check for conflicts
                let index = repo.index()?;
                if index.has_conflicts() {
                    warn!("Conflict during rebase of {}", branch_name);
                    return Ok(RebaseResult::Conflict {
                        branch: branch_name.to_string(),
                        onto: onto.to_string(),
                    });
                }

                // Commit the rebased changes
                if let Err(e) = rebase.commit(None, &repo.signature()?, None) {
                    // If commit fails due to empty commit, skip it
                    if e.code() == git2::ErrorCode::Applied {
                        debug!("Skipping empty commit during rebase");
                        continue;
                    }
                    return Err(e.into());
                }
            }
            Err(e) => {
                warn!("Rebase operation failed: {}", e);
                rebase.abort()?;
                return Err(e.into());
            }
        }
    }

    // Finish rebase
    rebase.finish(None)?;

    // Get new HEAD
    let new_branch = repo.find_branch(branch_name, BranchType::Local)?;
    let new_head = new_branch.get().peel_to_commit()?.id().to_string();

    info!("Rebased {} from {} to {}", branch_name, old_head, new_head);

    Ok(RebaseResult::Success {
        branch: branch_name.to_string(),
        old_head,
        new_head,
    })
}

/// Restack all branches that need it
pub fn restack_all(
    repo: &GitRepo,
    storage: &Storage,
    graph: &BranchGraph,
) -> Result<Vec<RebaseResult>> {
    let mut results = vec![];

    // Get branches in topological order (parents before children)
    let order: Vec<String> = graph.topological_order().iter().map(|s| s.to_string()).collect();

    for branch_name in &order {
        if let Some(info) = graph.get(branch_name) {
            // Skip if parent is trunk (handled differently)
            if graph.is_trunk(&info.parent) {
                // Rebase onto trunk
                match rebase_branch(repo, branch_name, graph.trunk()) {
                    Ok(result) => {
                        if let RebaseResult::Success { ref new_head, .. } = result {
                            // Update stored branch info
                            let _ = storage.update_branch(branch_name, |b| {
                                b.base_commit = Some(new_head.clone());
                            });
                        }
                        results.push(result);
                    }
                    Err(e) => {
                        warn!("Failed to restack {}: {}", branch_name, e);
                        return Err(e);
                    }
                }
            } else {
                // Rebase onto parent
                match rebase_branch(repo, branch_name, &info.parent) {
                    Ok(result) => {
                        if let RebaseResult::Conflict { ref branch, ref onto } = result {
                            // Save conflict state for continue/abort
                            storage.set_conflict(ConflictState {
                                branch: branch.clone(),
                                onto: onto.clone(),
                                original_commit: info.head_commit.clone().unwrap_or_default(),
                                remaining: order
                                    .iter()
                                    .skip_while(|b| *b != branch_name)
                                    .skip(1)
                                    .cloned()
                                    .collect(),
                            })?;
                            results.push(result);
                            return Ok(results); // Stop on conflict
                        }

                        if let RebaseResult::Success { ref new_head, .. } = result {
                            // Update stored branch info
                            let _ = storage.update_branch(branch_name, |b| {
                                b.head_commit = Some(new_head.clone());
                            });
                        }

                        results.push(result);
                    }
                    Err(e) => {
                        warn!("Failed to restack {}: {}", branch_name, e);
                        return Err(e);
                    }
                }
            }
        }
    }

    Ok(results)
}

/// Continue a rebase after conflict resolution
pub fn continue_rebase(repo: &GitRepo, storage: &Storage) -> Result<()> {
    let state = storage.load_state()?;

    let conflict = state
        .conflict_state
        .ok_or(Error::NoOperationInProgress)?;

    // Check if index is clean (conflicts resolved)
    let index = repo.index()?;
    if index.has_conflicts() {
        return Err(Error::RebaseConflict(conflict.branch.clone()));
    }

    // Continue the rebase
    // Note: In a full implementation, we'd need to handle the git2 rebase state
    // For now, we'll use a simpler approach

    storage.clear_conflict()?;

    // Continue with remaining branches
    // This would be called by the CLI to continue restacking

    Ok(())
}

/// Abort a rebase in progress
pub fn abort_rebase(repo: &GitRepo, storage: &Storage) -> Result<()> {
    let state = storage.load_state()?;

    let _conflict = state
        .conflict_state
        .ok_or(Error::NoOperationInProgress)?;

    // Abort the rebase
    // Reset to original state
    repo.cleanup_state()?;

    storage.clear_conflict()?;
    storage.complete_operation()?;

    Ok(())
}

/// Check if a branch needs rebasing onto its parent
pub fn needs_rebase(
    repo: &GitRepo,
    branch_name: &str,
    parent_name: &str,
) -> Result<bool> {
    let branch = repo.find_branch(branch_name, BranchType::Local)?;
    let parent = repo.find_branch(parent_name, BranchType::Local)?;

    let branch_commit = branch.get().peel_to_commit()?;
    let parent_commit = parent.get().peel_to_commit()?;

    // Find merge base
    let merge_base = repo.merge_base(branch_commit.id(), parent_commit.id())?;

    // If merge base is parent's HEAD, branch is up to date
    Ok(merge_base != parent_commit.id())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full rebase tests require a test repository setup
    // These would be integration tests

    #[test]
    fn test_rebase_result_variants() {
        let success = RebaseResult::Success {
            branch: "test".to_string(),
            old_head: "abc".to_string(),
            new_head: "def".to_string(),
        };

        match success {
            RebaseResult::Success { branch, .. } => assert_eq!(branch, "test"),
            _ => panic!("Expected Success"),
        }
    }
}
