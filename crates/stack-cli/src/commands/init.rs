//! Initialize Stack in a repository

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct InitArgs {
    /// Trunk branch name (auto-detected if not specified)
    #[arg(long)]
    trunk: Option<String>,
}

pub async fn execute(args: InitArgs) -> Result<()> {
    // Check if already initialized
    if Repository::open(".").is_ok() {
        output::warn("Stack is already initialized in this repository");
        return Ok(());
    }

    // Initialize
    let repo = Repository::init(".")?;

    let trunk = args.trunk.as_deref().unwrap_or(repo.trunk());

    output::success(&format!("Initialized Stack with trunk: {}", trunk));
    output::hint("Run 'gt create <branch>' to start a stack");

    Ok(())
}
