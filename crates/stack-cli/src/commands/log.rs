//! Show stack log

use anyhow::Result;
use clap::Args;
use stack_core::{Repository, stack::format_stack, Stack};

use crate::output;

#[derive(Args)]
pub struct LogArgs {
    /// Show all branches, not just current stack
    #[arg(long, short)]
    all: bool,
}

pub async fn execute(args: LogArgs, short: bool) -> Result<()> {
    let repo = Repository::open(".")?;

    let current = repo.current_branch()?;
    let graph = repo.load_graph()?;

    if args.all {
        // Show all tracked branches
        let branches = repo.storage().list_branches()?;

        if branches.is_empty() {
            output::info("No tracked branches");
            output::hint("Run 'gt create <name>' to start a stack");
            return Ok(());
        }

        output::info(&format!("Trunk: {}", repo.trunk()));
        output::info("");

        for branch in &branches {
            let is_current = current.as_ref() == Some(&branch.name);
            let marker = if is_current { "◉" } else { "○" };
            let pr_info = branch
                .pr_number
                .map(|n| format!(" (#{})", n))
                .unwrap_or_default();

            let depth = graph.depth(&branch.name);
            let indent = "  ".repeat(depth.saturating_sub(1));

            let name_str = output::branch(&branch.name, is_current);

            println!("{}{} {}{}", indent, marker, name_str, pr_info);
        }
    } else {
        // Show current stack
        let center = current.as_ref().ok_or_else(|| {
            anyhow::anyhow!("Not on a branch")
        })?;

        if !repo.storage().is_tracked(center) {
            output::warn(&format!("Branch '{}' is not tracked", center));
            output::hint(&format!("Run 'gt track {}' to start tracking", center));
            return Ok(());
        }

        // Load graph and create stack view
        let graph = repo.load_graph()?;
        let stack = Stack::from_graph(&graph, center, Some(center));

        if stack.is_empty() {
            output::info("Stack is empty");
            return Ok(());
        }

        let formatted = format_stack(&stack, short);
        print!("{}", formatted);
    }

    Ok(())
}

pub async fn execute_long(_args: LogArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    // Show full git log with graph
    let output = std::process::Command::new("git")
        .args(["log", "--all", "--graph", "--oneline", "--decorate", "-20"])
        .current_dir(repo.git().path().parent().unwrap_or(std::path::Path::new(".")))
        .output()?;

    print!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
