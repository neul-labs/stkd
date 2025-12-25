//! Status command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct StatusArgs {}

pub async fn execute(_args: StatusArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    // Check for pending operations
    if let Some(op) = repo.storage().current_operation()? {
        output::warn(&format!("Operation in progress: {}", op.name()));
        output::hint("Run 'gt continue' or 'gt abort'");
        return Ok(());
    }

    // Show current branch
    if let Some(branch) = repo.current_branch()? {
        println!("On branch: {}", output::branch(&branch, true));

        if repo.storage().is_tracked(&branch) {
            let info = repo.storage().load_branch(&branch)?.unwrap();
            println!("Parent: {}", info.parent);

            if let Some(pr) = info.pr_number {
                println!("PR: #{}", pr);
            }
        } else {
            output::hint(&format!("Branch '{}' is not tracked. Run 'gt track' to track it.", branch));
        }
    }

    // Check if clean
    if repo.is_clean()? {
        println!("\nWorking tree clean");
    } else {
        println!("\nYou have uncommitted changes");
    }

    Ok(())
}
