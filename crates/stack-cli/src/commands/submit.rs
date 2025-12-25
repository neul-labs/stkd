//! Submit command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct SubmitArgs {
    /// Submit entire stack (current + descendants)
    #[arg(long, short)]
    stack: bool,

    /// Create PRs as draft
    #[arg(long)]
    draft: bool,

    /// Just push, don't create PRs
    #[arg(long)]
    push_only: bool,
}

pub async fn execute(args: SubmitArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    let current = repo.current_branch()?.ok_or_else(|| {
        anyhow::anyhow!("Not on a branch")
    })?;

    let graph = repo.load_graph()?;

    // Get branches to submit
    let branches: Vec<String> = if args.stack {
        let mut to_submit = vec![current.clone()];
        to_submit.extend(
            graph.descendants(&current).iter().map(|s| s.to_string())
        );
        to_submit
    } else {
        vec![current.clone()]
    };

    output::info(&format!("Submitting {} branch(es)...", branches.len()));

    for branch in &branches {
        // Push branch
        output::info(&format!("  Pushing {}...", branch));

        let status = std::process::Command::new("git")
            .args(["push", "-u", "origin", branch, "--force-with-lease"])
            .status()?;

        if !status.success() {
            output::warn(&format!("Failed to push {}", branch));
            continue;
        }

        if args.push_only {
            output::success(&format!("Pushed {}", branch));
            continue;
        }

        // Check if PR exists
        let info = repo.storage().load_branch(branch)?;
        if let Some(ref info) = info {
            if info.pr_number.is_some() {
                output::success(&format!("Updated PR for {}", branch));
                continue;
            }
        }

        // TODO: Create PR using GitHub API
        output::info(&format!("  Would create PR for {}", branch));
        if args.draft {
            output::info("    (as draft)");
        }
    }

    output::success("Submit complete");
    output::hint("Run 'gt log' to see PR status");

    Ok(())
}
