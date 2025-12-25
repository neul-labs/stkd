//! Abort command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct AbortArgs {}

pub async fn execute(_args: AbortArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let state = repo.storage().load_state()?;

    if state.operation.is_none() && state.conflict_state.is_none() {
        output::info("No operation in progress");
        return Ok(());
    }

    // Abort any git operation
    let _ = std::process::Command::new("git")
        .args(["rebase", "--abort"])
        .status();

    // Clear state
    repo.storage().clear_conflict()?;
    repo.storage().complete_operation()?;

    output::success("Operation aborted");

    Ok(())
}
