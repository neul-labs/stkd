//! Abort command

use anyhow::Result;
use clap::Args;
use stkd_core::storage::OperationPhase;
use stkd_core::Repository;

use crate::output;

#[derive(Args)]
pub struct AbortArgs {}

pub async fn execute(_args: AbortArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let state = repo.storage().load_state()?;

    match &state.phase {
        OperationPhase::Idle | OperationPhase::Completed | OperationPhase::Aborted => {
            output::info("No operation in progress");
            return Ok(());
        }
        _ => {}
    }

    // Abort any git operation
    let _ = std::process::Command::new("git")
        .args(["rebase", "--abort"])
        .status();

    // Abort operation
    repo.storage().abort_operation()?;

    output::success("Operation aborted");

    Ok(())
}
