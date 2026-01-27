//! Show stack log

use anyhow::Result;
use clap::Args;
use colored::Colorize;
use stkd_core::{Repository, Stack};

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
        // Show all tracked branches organized by stack
        show_all_branches(&repo, current.as_deref())?;
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
        let stack = Stack::from_graph(&graph, center, Some(center));

        if stack.is_empty() {
            output::info("Stack is empty");
            return Ok(());
        }

        // Display stack with PR info
        display_stack(&repo, &stack, short)?;
    }

    Ok(())
}

fn show_all_branches(repo: &Repository, current: Option<&str>) -> Result<()> {
    let branches = repo.storage().list_branches()?;
    let graph = repo.load_graph()?;

    if branches.is_empty() {
        output::info("No tracked branches");
        output::hint("Run 'gt create <name>' to start a stack");
        return Ok(());
    }

    println!("{} {}", "Trunk:".dimmed(), repo.trunk());
    println!();

    // Group branches by their root (first branch from trunk)
    let mut roots: Vec<&str> = branches.iter()
        .filter(|b| graph.parent(&b.name) == Some(repo.trunk()))
        .map(|b| b.name.as_str())
        .collect();
    roots.sort();

    for root in roots {
        // Get the full stack for this root
        let stack_branches = graph.stack(root);

        for (i, branch_name) in stack_branches.iter().enumerate() {
            let branch_info = repo.storage().load_branch(branch_name)?;
            let is_current = current == Some(*branch_name);

            let marker = if is_current { "◉" } else { "○" };
            let marker_colored = if is_current {
                marker.green().to_string()
            } else {
                marker.dimmed().to_string()
            };

            let name_str = if is_current {
                branch_name.green().bold().to_string()
            } else {
                branch_name.to_string()
            };

            // PR info
            let pr_str = branch_info
                .as_ref()
                .and_then(|i| i.merge_request_id)
                .map(|n| format!(" {}", format!("#{}", n).cyan()))
                .unwrap_or_default();

            // Status indicator
            let status = if is_current { "[active]".green().to_string() } else { String::new() };

            // Indent based on position in stack
            let indent = if i > 0 { "  " } else { "" };
            let connector = if i > 0 && i == stack_branches.len() - 1 {
                "└─".dimmed().to_string()
            } else if i > 0 {
                "├─".dimmed().to_string()
            } else {
                "".to_string()
            };

            println!("{}{} {} {}{} {}", indent, connector, marker_colored, name_str, pr_str, status);
        }

        println!();
    }

    Ok(())
}

fn display_stack(repo: &Repository, stack: &Stack, short: bool) -> Result<()> {
    let entries: Vec<_> = stack.iter().collect();

    for (i, entry) in entries.iter().enumerate() {
        let is_current = entry.is_current();
        let is_tip = i == entries.len() - 1;

        // Markers
        let marker = if is_current { "◉" } else { "○" };
        let marker_colored = if is_current {
            marker.green().to_string()
        } else {
            marker.dimmed().to_string()
        };

        // Branch name
        let name_str = if is_current {
            entry.name().green().bold().to_string()
        } else {
            entry.name().to_string()
        };

        // Get branch info for PR
        let branch_info = repo.storage().load_branch(entry.name())?;

        // PR info
        let pr_str = branch_info
            .as_ref()
            .and_then(|i| i.merge_request_id)
            .map(|n| format!(" {}", output::mr_number(n)))
            .unwrap_or_default();

        // Status indicators
        let mut status_parts = Vec::new();
        if is_current {
            status_parts.push("[active]".green().to_string());
        }
        if is_tip {
            status_parts.push("[tip]".cyan().to_string());
        }
        let status = if status_parts.is_empty() {
            String::new()
        } else {
            format!(" {}", status_parts.join(" "))
        };

        // Connector for tree view
        let connector = if i == 0 && entries.len() == 1 {
            "─".dimmed()
        } else if i == entries.len() - 1 {
            "└".dimmed()
        } else if i == 0 {
            "┌".dimmed()
        } else {
            "│".dimmed()
        };

        if short {
            println!("{} {}{}", marker_colored, name_str, pr_str);
        } else {
            println!("{} {} {}{}{}", connector, marker_colored, name_str, pr_str, status);

            // Show PR URL if available and not in short mode
            if let Some(ref info) = branch_info {
                if let Some(ref url) = info.merge_request_url {
                    println!("{}   {}", "│".dimmed(), url.dimmed());
                }
            }
        }
    }

    Ok(())
}

pub async fn execute_long(_args: LogArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    // Show full git log with graph
    let out = std::process::Command::new("git")
        .args(["log", "--all", "--graph", "--oneline", "--decorate", "-20"])
        .current_dir(repo.git().path().parent().unwrap_or(std::path::Path::new(".")))
        .output()?;

    print!("{}", String::from_utf8_lossy(&out.stdout));

    Ok(())
}
