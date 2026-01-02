//! Sync command - sync with remote and update stack state

use anyhow::{Context, Result};
use clap::Args;
use stack_core::{rebase, Repository};
use std::time::Duration;

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

    /// Show what would be done without actually doing it
    #[arg(long)]
    dry_run: bool,

    /// Watch for changes and auto-sync (interval in seconds, default: 60)
    #[arg(long, value_name = "SECONDS")]
    watch: Option<Option<u64>>,
}

pub async fn execute(args: SyncArgs) -> Result<()> {
    // Watch mode - run sync periodically
    if let Some(interval) = args.watch {
        let interval_secs = interval.unwrap_or(60);
        output::info(&format!(
            "Watch mode enabled - syncing every {} seconds",
            interval_secs
        ));
        output::hint("Press Ctrl+C to stop");
        output::info("");

        loop {
            if let Err(e) = sync_once(&args).await {
                output::error(&format!("Sync failed: {}", e));
            }

            output::info("");
            output::info(&format!(
                "Next sync in {} seconds...",
                interval_secs
            ));
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
            output::info("");
        }
    }

    sync_once(&args).await
}

async fn sync_once(args: &SyncArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let trunk = repo.trunk().to_string();

    // Dry run mode - show what would be done
    if args.dry_run {
        output::info("Dry run - showing what would be done:");
        output::info("");

        if !args.no_pull {
            output::info(&format!("  {} Fetch from remote and update {}", output::ARROW, trunk));
        } else {
            output::info(&format!("  {} Fetch from remote", output::ARROW));
        }

        output::info(&format!("  {} Check MR status for tracked branches", output::ARROW));

        if !args.no_delete {
            output::info(&format!("  {} Delete branches with merged MRs", output::ARROW));
        }

        if !args.no_restack {
            let graph = repo.load_graph()?;
            let needs_restack = graph.needs_restack(repo.git())?;
            if !needs_restack.is_empty() || args.force {
                output::info(&format!("  {} Restack branches that need updating:", output::ARROW));
                for branch in &needs_restack {
                    output::info(&format!("      - {}", branch));
                }
                if needs_restack.is_empty() && args.force {
                    output::info("      (force flag set, will restack all)");
                }
            } else {
                output::info(&format!("  {} No branches need restacking", output::ARROW));
            }
        }

        output::info("");
        output::hint("Run without --dry-run to execute");
        return Ok(());
    }

    // Save current branch to return to it later
    let current_branch = repo.current_branch()?;

    // Step 1: Fetch from remote
    let spinner = output::spinner("Fetching from remote...");
    let result = std::process::Command::new("git")
        .args(["fetch", "origin", "--prune"])
        .output()
        .context("Failed to run git fetch")?;

    if !result.status.success() {
        output::finish_progress_error(&spinner, "Failed to fetch from remote");
        anyhow::bail!("Failed to fetch from remote");
    }
    output::finish_progress(&spinner, "Fetched from remote");

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
    let mr_spinner = output::spinner("Checking MR status...");

    let mut merged_branches = Vec::new();
    let mut closed_branches = Vec::new();
    let mut provider_connected = false;

    // Try to get MR status from provider
    match ProviderContext::from_repo(&repo).await {
        Ok(ctx) => {
            provider_connected = true;
            let branches = repo.storage().list_branches()?;

            for branch_info in branches {
                if let Some(mr_number) = branch_info.merge_request_id {
                    mr_spinner.set_message(format!("Checking MR #{}...", mr_number));
                    match ctx.provider().get_mr(&ctx.repo_id, mr_number.into()).await {
                        Ok(mr) => {
                            match mr.state {
                                stack_provider_api::MergeRequestState::Merged => {
                                    merged_branches.push(branch_info.name.clone());
                                }
                                stack_provider_api::MergeRequestState::Closed => {
                                    // Check if it was merged by looking if the branch was deleted
                                    let remote_ref = format!("refs/remotes/origin/{}", branch_info.name);
                                    if repo.git().find_reference(&remote_ref).is_err() {
                                        // Remote branch doesn't exist, MR was likely merged
                                        merged_branches.push(branch_info.name.clone());
                                    } else {
                                        closed_branches.push(branch_info.name.clone());
                                    }
                                }
                                _ => {
                                    // Still open, nothing to do
                                }
                            }
                        }
                        Err(_) => {
                            // Silently skip MRs we can't fetch
                        }
                    }
                }
            }
        }
        Err(e) => {
            output::finish_progress_error(&mr_spinner, "Could not connect to provider");
            output::hint(&format!("Error: {}. Continuing without MR status update", e));
        }
    }

    // Finish the spinner if we connected successfully
    if provider_connected {
        if !merged_branches.is_empty() || !closed_branches.is_empty() {
            output::finish_progress(
                &mr_spinner,
                &format!(
                    "Found {} merged, {} closed MR(s)",
                    merged_branches.len(),
                    closed_branches.len()
                ),
            );
        } else {
            output::finish_progress(&mr_spinner, "All MRs up to date");
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
