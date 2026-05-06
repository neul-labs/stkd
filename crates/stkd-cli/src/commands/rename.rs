//! Rename branch command

use anyhow::Result;
use clap::Args;
use stkd_core::Repository;

use crate::output;

#[derive(Args)]
pub struct RenameArgs {
    /// New name for the branch
    name: String,
}

pub async fn execute(args: RenameArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let old_name = repo
        .current_branch()?
        .ok_or_else(|| anyhow::anyhow!("Not on a branch"))?;

    let info = repo.rename_branch(&args.name)?;

    output::success(&format!(
        "Renamed '{}' to '{}'",
        old_name,
        output::branch(&info.name, true)
    ));

    Ok(())
}
