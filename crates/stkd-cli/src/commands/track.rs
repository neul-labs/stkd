//! Track branch command

use anyhow::Result;
use clap::Args;
use stkd_core::Repository;

use crate::output;

#[derive(Args)]
pub struct TrackArgs {
    /// Branch name to track
    branch: Option<String>,
}

pub async fn execute(args: TrackArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let branch = args.branch.or_else(|| repo.current_branch().ok().flatten())
        .ok_or_else(|| anyhow::anyhow!("No branch specified and not on a branch"))?;

    let info = repo.track_branch(&branch)?;

    output::success(&format!(
        "Now tracking '{}' with parent '{}'",
        output::branch(&info.name, true),
        info.parent
    ));

    Ok(())
}
