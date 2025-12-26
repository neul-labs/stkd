//! Sync command - sync with remote and update stack state

use anyhow::{Context, Result};
use clap::Args;
use stack_core::{rebase, Repository};

use crate::output;
use crate::provider_context::ProviderContext;

#[derive(Args)]
pub struct SyncArgs {
    /// Don't delete merged branches
    #[arg(long)]
    no_delete: bool,

    /// Don't restack after sync
    #[arg(long)]
    no_restack: bool,

    /// Don't update trunk
    #[arg(long)]
    no_pull: bool,

    /// Force restack even if not needed
    #[arg(long)]
    force: bool,
}

pub async fn execute(args: SyncArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let trunk = repo.trunk().to_string();

    // Save current branch to return to it later
    let current_branch = repo.current_branch()?;

    // Step 1: Fetch from remote
    output::info("Fetching from remote...");
    let status = std::process::Command::new("git")
        .args(["fetch", "origin", "--prune"])
        .status()
        .context("Failed to run git fetch")?;

    if !status.success() {
        anyhow::bail!("Failed to fetch from remote");
    }
    output::success("Fetched from remote");

    // Step 2: Update trunk
    if !args.no_pull {
        output::info(&format!("Updating {}...", trunk));

        // Checkout trunk
        repo.checkout(&trunk)?;

        // Pull with fast-forward only
        let status = std::process::Command::new("git")
            .args(["pull", "--ff-only", "origin", &trunk])
            .status()
            .context("Failed to run git pull")?;

        if !status.success() {
            output::warn(&format!("Could not fast-forward {}. You may need to resolve conflicts manually.", trunk));
        } else {
            output::success(&format!("Updated {}", trunk));
        }

        // Return to original branch if it still exists
        if let Some(ref branch) = current_branch {
            if branch != &trunk {
                if repo.git().find_branch(branch, git2::BranchType::Local).is_ok() {
                    repo.checkout(branch)?;
                }
            }
        }
    }

    // Step 3: Update MR status from provider
    output::info("Checking MR status...");

    let mut merged_branches = Vec::new();
    let mut closed_branches = Vec::new();

    // Try to get MR status from provider
    match ProviderContext::from_repo(&repo).await {
        Ok(ctx) => {
            let branches = repo.storage().list_branches()?;

            for branch_info in branches {
                if let Some(mr_number) = branch_info.merge_request_id {
                    match ctx.provider().get_mr(&ctx.repo_id, mr_number.into()).await {
                        Ok(mr) => {
                            match mr.state {
                                stack_provider_api::MergeRequestState::Merged => {
                                    merged_branches.push(branch_info.name.clone());
                                    output::info(&format!("  {} MR #{} was merged", output::ARROW, mr_number));
                                }
                                stack_provider_api::MergeRequestState::Closed => {
                                    // Check if it was merged by looking if the branch was deleted
                                    let remote_ref = format!("refs/remotes/origin/{}", branch_info.name);
                                    if repo.git().find_reference(&remote_ref).is_err() {
                                        // Remote branch doesn't exist, MR was likely merged
                                        merged_branches.push(branch_info.name.clone());
                                        output::info(&format!("  {} MR #{} was merged", output::ARROW, mr_number));
                                    } else {
                                        closed_branches.push(branch_info.name.clone());
                                        output::info(&format!("  {} MR #{} was closed", output::ARROW, mr_number));
                                    }
                                }
                                _ => {
                                    // Still open, nothing to do
                                }
                            }
                        }
                        Err(e) => {
                            output::warn(&format!("Could not fetch MR #{} status: {}", mr_number, e));
                        }
                    }
                }
            }
        }
        Err(e) => {
            output::warn(&format!("Could not connect to provider: {}", e));
            output::hint("Continuing without MR status update");
        }
    }

    // Step 4: Clean up merged branches
    if !args.no_delete && !merged_branches.is_empty() {
        output::info(&format!("\nCleaning up {} merged branch(es)...", merged_branches.len()));

        for branch in &merged_branches {
            output::info(&format!("  {} Deleting {}...", output::ARROW, branch));

            // Delete local branch
            if let Err(e) = std::process::Command::new("git")
                .args(["branch", "-D", branch])
                .status()
            {
                output::warn(&format!("Failed to delete {}: {}", branch, e));
                continue;
            }

            // Untrack the branch
            if let Err(e) = repo.untrack_branch(branch) {
                output::warn(&format!("Failed to untrack {}: {}", branch, e));
            }

            output::success(&format!("Deleted {}", branch));
        }
    }

    // Step 5: Restack branches
    if !args.no_restack {
        let graph = repo.load_graph()?;

        // Find branches that need restacking
        let needs_restack = graph.needs_restack(repo.git())?;

        if !needs_restack.is_empty() || args.force {
            output::info("\nRestacking branches...");

            match rebase::restack_all(repo.git(), repo.storage(), &graph) {
                Ok(results) => {
                    for result in results {
                        match result {
                            rebase::RebaseResult::Success { branch, .. } => {
                                output::success(&format!("Restacked {}", branch));
                            }
                            rebase::RebaseResult::UpToDate { branch } => {
                                output::info(&format!("  {} {} is up to date", output::ARROW, branch));
                            }
                            rebase::RebaseResult::Conflict { branch, onto } => {
                                output::warn(&format!("Conflict restacking {} onto {}", branch, onto));
                                output::hint("Resolve conflicts and run 'gt continue'");
                                return Ok(());
                            }
                        }
                    }
                }
                Err(e) => {
                    output::error(&format!("Restack failed: {}", e));
                    output::hint("Run 'gt abort' to cancel or resolve manually");
                    return Err(e.into());
                }
            }
        } else {
            output::info("All branches are up to date");
        }
    }

    // Return to original branch or first available stacked branch
    if let Some(ref branch) = current_branch {
        if !merged_branches.contains(branch) {
            if repo.git().find_branch(branch, git2::BranchType::Local).is_ok() {
                repo.checkout(branch)?;
            }
        } else {
            // Original branch was merged, go to trunk
            output::info(&format!("Branch {} was merged, switching to {}", branch, trunk));
            repo.checkout(&trunk)?;
        }
    }

    output::success("Sync complete");

    Ok(())
}
