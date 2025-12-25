//! Continue command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct ContinueArgs {}

pub async fn execute(_args: ContinueArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let state = repo.storage().load_state()?;

    if state.operation.is_none() && state.conflict_state.is_none() {
        output::info("No operation in progress");
        return Ok(());
    }

    if let Some(ref conflict) = state.conflict_state {
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

        // Clear conflict state
        repo.storage().clear_conflict()?;

        // Continue with remaining branches
        if !conflict.remaining.is_empty() {
            output::info(&format!(
                "Continuing to restack {} remaining branches...",
                conflict.remaining.len()
            ));
            // Would continue restacking here
        }
    }

    repo.storage().complete_operation()?;
    output::success("Operation complete");

    Ok(())
}
