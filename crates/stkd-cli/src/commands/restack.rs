//! Restack command - rebase branches onto their parents

use anyhow::Result;
use clap::Args;
use stkd_core::{rebase, Repository};

use crate::output;

#[derive(Args)]
pub struct RestackArgs {
    /// Only restack current branch and descendants
    #[arg(long)]
    current_only: bool,

    /// Force restack even if branches appear up-to-date
    #[arg(long, short)]
    force: bool,

    /// Show what would be done without actually doing it
    #[arg(long)]
    dry_run: bool,
}

pub async fn execute(args: RestackArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let graph = repo.load_graph()?;

    // Get branches to restack
    let all_branches: Vec<String> = if args.current_only {
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

    // Filter to branches that actually need restacking (unless --force)
    let branches: Vec<String> = if args.force {
        all_branches
    } else {
        let needs_restack = graph.needs_restack(repo.git())?;
        all_branches
            .into_iter()
            .filter(|b| needs_restack.contains(b))
            .collect()
    };

    if branches.is_empty() {
        output::success("All branches are up to date");
        return Ok(());
    }

    // Dry run mode
    if args.dry_run {
        output::info("Dry run - showing what would be done:");
        output::info("");

        for branch in &branches {
            let info = repo.storage().load_branch(branch)?;
            if let Some(info) = info {
                let onto = if graph.is_trunk(&info.parent) {
                    repo.trunk().to_string()
                } else {
                    info.parent.clone()
                };
                output::info(&format!("  {} Rebase {} onto {}", output::ARROW, branch, onto));
            }
        }

        output::info("");
        output::hint("Run without --dry-run to execute");
        return Ok(());
    }

    // Ensure clean working tree before rebasing
    repo.ensure_clean()?;

    let pb = output::progress_bar(branches.len() as u64, "Restacking");

    for branch in &branches {
        let info = repo.storage().load_branch(branch)?;
        if let Some(info) = info {
            let onto = if graph.is_trunk(&info.parent) {
                repo.trunk().to_string()
            } else {
                info.parent.clone()
            };

            pb.set_message(format!("{} onto {}", branch, onto));

            match rebase::rebase_branch(repo.git(), branch, &onto) {
                Ok(rebase::RebaseResult::Success { .. }) => {
                    pb.inc(1);
                }
                Ok(rebase::RebaseResult::UpToDate { .. }) => {
                    pb.inc(1);
                }
                Ok(rebase::RebaseResult::Conflict { .. }) => {
                    output::finish_progress_error(&pb, &format!("Conflict in {}", branch));
                    output::hint("Resolve conflicts and run 'gt continue'");
                    return Ok(());
                }
                Err(e) => {
                    output::finish_progress_error(&pb, &format!("Failed to restack {}", branch));
                    return Err(e.into());
                }
            }
        }
    }

    output::finish_progress(&pb, &format!("Restacked {} branches", branches.len()));

    Ok(())
}
