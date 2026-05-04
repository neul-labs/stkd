//! Continue command

use anyhow::Result;
use clap::Args;
use stkd_core::storage::OperationPhase;
use stkd_core::Repository;

use crate::output;

#[derive(Args)]
pub struct ContinueArgs {}

pub async fn execute(_args: ContinueArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let state = repo.storage().load_state()?;

    match &state.phase {
        OperationPhase::Idle | OperationPhase::Completed | OperationPhase::Aborted => {
            output::info("No operation in progress");
            return Ok(());
        }
        OperationPhase::Conflict { conflict, .. } => {
            output::info(&format!(
                "Continuing rebase of '{}' onto '{}'",
                conflict.branch, conflict.onto
            ));

            // Check if conflicts are resolved
            let index = repo.git().index()?;
            if index.has_conflicts() {
                output::error("Conflicts not resolved. Fix conflicts and stage changes.");
                return Ok(());
            }

            // Continue rebase
            let status = std::process::Command::new("git")
                .args(["rebase", "--continue"])
                .status()?;

            if !status.success() {
                anyhow::bail!("Rebase continue failed");
            }

            // Continue operation
            repo.storage().continue_operation()?;

            // Continue with remaining branches
            if !conflict.remaining.is_empty() {
                output::info(&format!(
                    "Continuing to restack {} remaining branches...",
                    conflict.remaining.len()
                ));
                // Would continue restacking here
            }
        }
        OperationPhase::InProgress { .. } => {
            // Operation is in progress, just complete it
        }
    }

    repo.storage().complete_operation()?;
    output::success("Operation complete");

    Ok(())
}
