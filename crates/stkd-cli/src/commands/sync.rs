//! Sync command - sync with remote and update stack state

use anyhow::{Context, Result};
use clap::Args;
use stkd_core::Repository;
use std::time::Duration;

use crate::output;

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

pub async fn execute(args: SyncArgs, json: bool) -> Result<()> {
    // Watch mode - run sync periodically
    if let Some(interval) = args.watch {
        let interval_secs = interval.unwrap_or(60);
        if !json {
            output::info(&format!(
                "Watch mode enabled - syncing every {} seconds",
                interval_secs
            ));
            output::hint("Press Ctrl+C to stop");
            output::info("");
        }

        loop {
            if let Err(e) = sync_once(&args, json).await {
                if json {
                    eprintln!("{}", serde_json::to_string_pretty(&serde_json::json!({"error": format!("{}", e)})).unwrap_or_default());
                } else {
                    output::error(&format!("Sync failed: {}", e));
                }
            }

            if !json {
                output::info("");
                output::info(&format!(
                    "Next sync in {} seconds...",
                    interval_secs
                ));
            }
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
            if !json {
                output::info("");
            }
        }
    }

    sync_once(&args, json).await
}

async fn sync_once(args: &SyncArgs, json: bool) -> Result<()> {
    let repo = Repository::open(".")?;
    let trunk = repo.trunk().to_string();

    if args.dry_run {
        if json {
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({"dry_run": true, "actions": ["fetch", "update_trunk", "check_mr_status", "delete_merged", "restack"]}))?);
            return Ok(());
        }

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

    let opts = stkd_engine::SyncOptions {
        no_delete: args.no_delete,
        no_restack: args.no_restack,
        no_pull: args.no_pull,
        force: args.force,
        dry_run: false,
    };

    let provider = stkd_engine::ProviderContext::from_repo(&repo).await.ok();
    let (provider_ref, repo_id_ref) = match &provider {
        Some(ctx) => (Some(ctx.provider()), Some(&ctx.repo_id)),
        None => (None, None),
    };

    let result = stkd_engine::sync(&repo, opts, provider_ref, repo_id_ref).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        // Human output
        if result.fetched {
            output::success("Fetched from remote");
        }
        if result.trunk_updated {
            output::success(&format!("Updated {}", trunk));
        }
        if !result.merged_branches.is_empty() {
            output::info(&format!("Found {} merged branch(es)", result.merged_branches.len()));
            for branch in &result.merged_branches {
                output::info(&format!("  {} Deleting {}...", output::ARROW, branch));
            }
        }
        for entry in &result.restacked {
            if entry.up_to_date {
                output::info(&format!("  {} {} is up to date", output::ARROW, entry.branch));
            } else {
                output::success(&format!("Restacked {}", entry.branch));
            }
        }
        for conflict in &result.conflicts {
            output::warn(&format!("Conflict restacking {} onto {}", conflict.branch, conflict.onto));
        }
        if result.conflicts.is_empty() {
            output::success("Sync complete");
        }
    }

    Ok(())
}
