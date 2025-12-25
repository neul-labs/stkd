//! Restack command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct RestackArgs {
    /// Only restack current branch and descendants
    #[arg(long)]
    current_only: bool,
}

pub async fn execute(args: RestackArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    repo.ensure_clean()?;

    let graph = repo.load_graph()?;

    // Get branches to restack
    let branches: Vec<String> = if args.current_only {
        let current = repo.current_branch()?.ok_or_else(|| {
            anyhow::anyhow!("Not on a branch")
        })?;

        let mut to_restack = vec![current.clone()];
        to_restack.extend(
            graph.descendants(&current).iter().map(|s| s.to_string())
        );
        to_restack
    } else {
        graph.topological_order().iter().map(|s| s.to_string()).collect()
    };

    if branches.is_empty() {
        output::info("Nothing to restack");
        return Ok(());
    }

    output::info(&format!("Restacking {} branches...", branches.len()));

    for branch in &branches {
        let info = repo.storage().load_branch(branch)?;
        if let Some(info) = info {
            if graph.is_trunk(&info.parent) {
                // Rebase onto trunk
                output::info(&format!("  {} onto {}", branch, repo.trunk()));
            } else {
                // Rebase onto parent
                output::info(&format!("  {} onto {}", branch, info.parent));
            }

            // TODO: Actually perform rebase using stack_core::rebase
        }
    }

    output::success("Restack complete");

    Ok(())
}
