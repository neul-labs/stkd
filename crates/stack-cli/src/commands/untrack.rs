//! Untrack branch command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct UntrackArgs {
    /// Branch name to untrack (defaults to current)
    branch: Option<String>,
}

pub async fn execute(args: UntrackArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let branch = args.branch.or_else(|| repo.current_branch().ok().flatten())
        .ok_or_else(|| anyhow::anyhow!("No branch specified and not on a branch"))?;

    repo.untrack_branch(&branch)?;

    output::success(&format!("Untracked branch '{}'", branch));

    Ok(())
}
