//! Checkout command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct CheckoutArgs {
    /// Branch name to checkout
    branch: Option<String>,
}

pub async fn execute(args: CheckoutArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    repo.ensure_clean()?;

    let branch = if let Some(name) = args.branch {
        name
    } else {
        // Interactive selection
        let branches = repo.storage().list_branches()?;
        if branches.is_empty() {
            anyhow::bail!("No tracked branches");
        }

        let options: Vec<&str> = branches.iter().map(|b| b.name.as_str()).collect();
        let idx = output::select("Select branch", &options)
            .ok_or_else(|| anyhow::anyhow!("No branch selected"))?;

        branches[idx].name.clone()
    };

    repo.checkout(&branch)?;
    output::success(&format!("Switched to {}", output::branch(&branch, true)));

    Ok(())
}
