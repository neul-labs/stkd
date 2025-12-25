//! Land command

use anyhow::Result;
use clap::Args;

use crate::output;

#[derive(Args)]
pub struct LandArgs {
    /// Merge method (merge, squash, rebase)
    #[arg(long, default_value = "squash")]
    method: String,
}

pub async fn execute(args: LandArgs) -> Result<()> {
    // TODO: Implement landing via GitHub API

    output::info(&format!("Would land with method: {}", args.method));
    output::warn("Landing is not yet implemented");
    output::hint("Use GitHub web UI or 'gh pr merge' for now");

    Ok(())
}
