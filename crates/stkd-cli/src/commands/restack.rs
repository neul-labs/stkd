//! Restack command - rebase branches onto their parents

use anyhow::Result;
use clap::Args;
use stkd_core::Repository;

use crate::output;

#[derive(Args)]
pub struct RestackArgs {
    /// Only restack current branch and descendants
    #[arg(long)]
    current_only: bool,

    /// Force restack even if branches appear up-to-date
    #[arg(long, short)]
    force: bool,

    /// Show what would be done without actually doing it
    #[arg(long)]
    dry_run: bool,
}

pub async fn execute(args: RestackArgs, json: bool) -> Result<()> {
    let repo = Repository::open(".")?;

    let opts = stkd_engine::RestackOptions {
        current_only: args.current_only,
        force: args.force,
        dry_run: args.dry_run,
    };

    let result = stkd_engine::restack(&repo, opts)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        for entry in &result.restacked {
            match entry.status {
                stkd_engine::RestackStatus::Success => {
                    output::success(&format!("Restacked {}", entry.branch));
                }
                stkd_engine::RestackStatus::UpToDate => {
                    output::info(&format!(
                        "  {} {} is up to date",
                        output::ARROW,
                        entry.branch
                    ));
                }
                stkd_engine::RestackStatus::Conflict => {
                    output::warn(&format!(
                        "Conflict restacking {} onto {}",
                        entry.branch, entry.onto
                    ));
                    output::hint("Resolve conflicts and run 'gt continue'");
                }
                stkd_engine::RestackStatus::Error => {
                    output::error(&format!("Failed to restack {}", entry.branch));
                }
            }
        }

        if result.restacked.is_empty() {
            output::success("All branches are up to date");
        } else if !result.restacked.iter().any(|e| {
            e.status == stkd_engine::RestackStatus::Conflict
                || e.status == stkd_engine::RestackStatus::Error
        }) {
            output::success("Restack complete");
        }
    }

    Ok(())
}
