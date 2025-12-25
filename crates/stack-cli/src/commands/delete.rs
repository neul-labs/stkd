//! Delete branch command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct DeleteArgs {
    /// Branch name to delete
    branch: String,

    /// Force delete even if branch has children
    #[arg(long, short)]
    force: bool,
}

pub async fn execute(args: DeleteArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    if !args.force {
        if !output::confirm(&format!("Delete branch '{}'?", args.branch)) {
            output::info("Cancelled");
            return Ok(());
        }
    }

    repo.delete_branch(&args.branch, args.force)?;

    output::success(&format!("Deleted branch '{}'", args.branch));

    Ok(())
}
