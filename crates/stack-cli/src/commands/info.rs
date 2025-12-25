//! Info command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct InfoArgs {
    /// Branch to show info for (defaults to current)
    branch: Option<String>,
}

pub async fn execute(args: InfoArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let branch = args.branch.or_else(|| repo.current_branch().ok().flatten())
        .ok_or_else(|| anyhow::anyhow!("No branch specified and not on a branch"))?;

    let info = repo.storage().load_branch(&branch)?
        .ok_or_else(|| anyhow::anyhow!("Branch '{}' is not tracked", branch))?;

    println!("Branch: {}", output::branch(&info.name, true));
    println!("Parent: {}", info.parent);
    println!("Status: {}", info.status);

    if !info.children.is_empty() {
        println!("Children: {}", info.children.join(", "));
    }

    if let Some(pr) = info.pr_number {
        println!("PR: #{}", pr);
        if let Some(ref url) = info.pr_url {
            println!("URL: {}", url);
        }
    }

    println!("Created: {}", info.created_at.format("%Y-%m-%d %H:%M"));
    println!("Updated: {}", info.updated_at.format("%Y-%m-%d %H:%M"));

    Ok(())
}
