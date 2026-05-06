//! Modify (amend) command

use anyhow::Result;
use clap::Args;

use crate::output;

#[derive(Args)]
pub struct ModifyArgs {
    /// Commit message
    #[arg(short, long)]
    message: Option<String>,

    /// Amend without editing the message
    #[arg(long)]
    no_edit: bool,

    /// Create a new commit instead of amending
    #[arg(long)]
    commit: bool,
}

pub async fn execute(args: ModifyArgs) -> Result<()> {
    // Use git directly for now
    let mut cmd = std::process::Command::new("git");

    if args.commit {
        cmd.arg("commit");
        if let Some(msg) = &args.message {
            cmd.args(["-m", msg]);
        }
    } else {
        cmd.args(["commit", "--amend"]);
        if let Some(msg) = &args.message {
            cmd.args(["-m", msg]);
        } else {
            cmd.arg("--no-edit");
        }
    }

    let status = cmd.status()?;

    if status.success() {
        if args.commit {
            output::success("Created new commit");
        } else {
            output::success("Amended commit");
        }
        output::hint("Run 'gt restack' to update dependent branches");
    }

    Ok(())
}
