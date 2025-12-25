//! Sync command

use anyhow::Result;
use clap::Args;

use crate::output;

#[derive(Args)]
pub struct SyncArgs {
    /// Don't delete merged branches
    #[arg(long)]
    no_delete: bool,

    /// Don't restack after sync
    #[arg(long)]
    no_restack: bool,
}

pub async fn execute(args: SyncArgs) -> Result<()> {
    // Fetch from remote
    output::info("Fetching from remote...");

    let status = std::process::Command::new("git")
        .args(["fetch", "origin"])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to fetch from remote");
    }

    // TODO: Sync PR status from GitHub
    // TODO: Identify merged branches
    // TODO: Delete merged branches (with confirmation)
    // TODO: Restack if needed

    if !args.no_delete {
        output::info("Checking for merged branches...");
        // Placeholder for merged branch detection
    }

    if !args.no_restack {
        output::info("Restacking branches...");
        // Would call restack here
    }

    output::success("Sync complete");

    Ok(())
}
