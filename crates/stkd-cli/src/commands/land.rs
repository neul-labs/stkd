//! Land command - merge MRs and clean up branches

use anyhow::{Context, Result};
use clap::Args;
use stkd_core::Repository;
use stkd_provider_api::MergeMethod;

use crate::output;
use crate::provider_context::ProviderContext;

#[derive(Args)]
pub struct LandArgs {
    /// Merge method (merge, squash, rebase)
    #[arg(long, default_value = "squash")]
    method: String,

    /// Land the entire stack from bottom to top
    #[arg(long, short)]
    stack: bool,

    /// Delete local branches after landing
    #[arg(long, default_value = "true")]
    delete_local: bool,

    /// Don't sync after landing
    #[arg(long)]
    no_sync: bool,

    /// Don't confirm before landing
    #[arg(long, short = 'y')]
    yes: bool,

    /// Show what would be done without actually doing it
    #[arg(long)]
    dry_run: bool,
}

pub async fn execute(args: LandArgs) -> Result<()> {
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

    // Get branch info
    let info = repo.storage()
        .load_branch(&current)?
        .context("Branch info not found")?;

    // Must have an MR to land
    let mr_number = info.merge_request_id.ok_or_else(|| {
        anyhow::anyhow!("No MR found for branch '{}'. Run 'gt submit' first.", current)
    })?;

    // Parse merge method
    let merge_method = match args.method.to_lowercase().as_str() {
        "merge" => MergeMethod::Merge,
        "squash" => MergeMethod::Squash,
        "rebase" => MergeMethod::Rebase,
        "ff" | "fast-forward" => MergeMethod::FastForward,
        _ => anyhow::bail!("Invalid merge method: {}. Use 'merge', 'squash', 'rebase', or 'ff'.", args.method),
    };

    // Create provider context
    let ctx = ProviderContext::from_repo(&repo).await?;

    // Get branches to land
    let branches_to_land: Vec<(String, u64)> = if args.stack {
        // Land from bottom of stack to current (ancestors + current)
        let mut to_land = Vec::new();
        let ancestors = graph.ancestors(&current);

        // Add ancestors from bottom (reversed)
        for ancestor in ancestors.into_iter().rev() {
            if let Some(info) = repo.storage().load_branch(ancestor)? {
                if let Some(mr_num) = info.merge_request_id {
                    to_land.push((ancestor.to_string(), mr_num));
                }
            }
        }

        // Add current
        to_land.push((current.clone(), mr_number));
        to_land
    } else {
        vec![(current.clone(), mr_number)]
    };

    // Dry run mode
    if args.dry_run {
        output::info("Dry run - showing what would be done:");
        output::info("");
        output::info("Branches to land:");
        for (branch, mr_num) in &branches_to_land {
            output::info(&format!("  {} Merge MR #{} for {}", output::ARROW, mr_num, branch));
        }
        output::info(&format!("\nMerge method: {}", args.method));
        output::info(&format!("Provider: {}", ctx.provider_type));
        if args.delete_local {
            output::info("Will delete local branches after landing");
        }
        if !args.no_sync {
            output::info(&format!("Will sync and switch to {}", repo.trunk()));
        }
        output::info("");
        output::hint("Run without --dry-run to execute");
        return Ok(());
    }

    // Confirm landing
    if !args.yes {
        output::info("Branches to land:");
        for (branch, mr_num) in &branches_to_land {
            output::info(&format!("  {} {} (MR #{})", output::ARROW, branch, mr_num));
        }
        output::info(&format!("\nMerge method: {}", args.method));
        output::info(&format!("Provider: {}", ctx.provider_type));

        if !output::confirm("Proceed with landing?") {
            output::info("Aborted.");
            return Ok(());
        }
    }

    // Land each branch
    for (branch, mr_num) in &branches_to_land {
        output::info(&format!("Landing {} (MR #{})...", branch, mr_num));

        // Merge the MR
        match ctx.provider().merge_mr(&ctx.repo_id, (*mr_num).into(), merge_method).await {
            Ok(result) => {
                if result.merged {
                    output::success(&format!("Merged MR #{} for {}", mr_num, branch));
                } else {
                    output::warn(&format!("MR #{} was not merged: {}", mr_num, result.message));
                    continue;
                }
            }
            Err(e) => {
                output::error(&format!("Failed to merge MR #{}: {}", mr_num, e));
                anyhow::bail!("Landing failed at branch '{}'", branch);
            }
        }

        // Delete local branch if requested
        if args.delete_local && *branch != current {
            output::info(&format!("  {} Deleting local branch {}...", output::ARROW, branch));
            if let Err(e) = repo.delete_branch(branch, true) {
                output::warn(&format!("Failed to delete local branch {}: {}", branch, e));
            }
        }
    }

    // Sync with remote to update state
    if !args.no_sync {
        output::info("Syncing with remote...");

        let status = std::process::Command::new("git")
            .args(["fetch", "origin"])
            .status()
            .context("Failed to run git fetch")?;

        if !status.success() {
            output::warn("Failed to fetch from remote");
        }

        // Checkout trunk if we landed the current branch
        if branches_to_land.iter().any(|(b, _)| b == &current) {
            output::info(&format!("Switching to {}...", repo.trunk()));
            repo.checkout(repo.trunk())?;

            // Pull trunk
            let status = std::process::Command::new("git")
                .args(["pull", "--ff-only"])
                .status()
                .context("Failed to run git pull")?;

            if !status.success() {
                output::warn("Failed to pull trunk");
            }

            // Delete the landed branch locally
            if args.delete_local {
                output::info(&format!("Deleting local branch {}...", current));
                let _ = std::process::Command::new("git")
                    .args(["branch", "-D", &current])
                    .status();

                // Untrack the branch
                let _ = repo.untrack_branch(&current);
            }
        }
    }

    output::success("Landing complete!");

    // Show what to do next
    let children = graph.children(&current);
    if !children.is_empty() {
        output::hint(&format!(
            "Child branches ({}) may need restacking. Run 'gt sync'",
            children.join(", ")
        ));
    }

    Ok(())
}
