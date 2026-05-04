//! Land command - merge MRs and clean up branches

use anyhow::{Context, Result};
use clap::Args;
use stkd_core::Repository;

use crate::output;

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

pub async fn execute(args: LandArgs, json: bool) -> Result<()> {
    let repo = Repository::open(".")?;
    let ctx = stkd_engine::ProviderContext::from_repo(&repo).await?;

    let opts = stkd_engine::LandOptions {
        method: args.method,
        stack: args.stack,
        delete_local: args.delete_local,
        no_sync: args.no_sync,
        dry_run: args.dry_run,
    };

    if !json && !args.dry_run && !args.yes {
        let graph = repo.load_graph()?;
        let current = repo.current_branch()?.ok_or_else(|| {
            anyhow::anyhow!("Not on a branch")
        })?;

        let mut to_land = Vec::new();
        if args.stack {
            let ancestors = graph.ancestors(&current);
            for ancestor in ancestors.iter().rev() {
                if let Some(info) = repo.storage().load_branch(ancestor)? {
                    if let Some(mr_num) = info.merge_request_id {
                        to_land.push((ancestor.to_string(), mr_num));
                    }
                }
            }
        }

        let info = repo.storage().load_branch(&current)?.ok_or_else(|| {
            anyhow::anyhow!("Branch info not found")
        })?;
        if let Some(mr_num) = info.merge_request_id {
            to_land.push((current.clone(), mr_num));
        }

        output::info("Branches to land:");
        for (branch, mr_num) in &to_land {
            output::info(&format!("  {} {} (MR #{})", output::ARROW, branch, mr_num));
        }
        output::info(&format!("\nMerge method: {}", opts.method));
        output::info(&format!("Provider: {}", ctx.provider_type));

        if !output::confirm("Proceed with landing?") {
            output::info("Aborted.");
            return Ok(());
        }
    }

    let result = stkd_engine::land(&repo, opts, ctx.provider(), &ctx.repo_id).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        for landed in &result.landed {
            if landed.merged {
                output::success(&format!("Merged MR #{} for {}", landed.mr_number, landed.branch));
            } else {
                output::warn(&format!("MR #{} was not merged: {}", landed.mr_number, landed.message));
            }
        }
        for deleted in &result.deleted {
            output::info(&format!("  {} Deleted local branch {}", output::ARROW, deleted));
        }
        if result.current_switched_to_trunk {
            output::info(&format!("Switched to {}", repo.trunk()));
        }
        output::success("Landing complete!");
    }

    Ok(())
}
