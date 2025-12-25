//! Create a new branch

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct CreateArgs {
    /// Name for the new branch
    name: String,

    /// Create branch from trunk instead of current branch
    #[arg(long)]
    from_trunk: bool,
}

pub async fn execute(args: CreateArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    // Ensure clean working directory
    repo.ensure_clean()?;

    // If from_trunk, checkout trunk first
    if args.from_trunk {
        repo.checkout(repo.trunk())?;
    }

    // Create the branch
    let info = repo.create_branch(&args.name)?;

    output::success(&format!(
        "Created branch '{}' on top of '{}'",
        output::branch(&info.name, true),
        info.parent
    ));

    Ok(())
}
