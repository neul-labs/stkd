//! Status command - show current stack status

use anyhow::Result;
use clap::Args;
use colored::Colorize;
use stack_core::Repository;

use crate::output;
use crate::provider_context::ProviderContext;

#[derive(Args)]
pub struct StatusArgs {
    /// Show detailed MR information
    #[arg(long, short)]
    verbose: bool,

    /// Fetch latest status from provider
    #[arg(long)]
    fetch: bool,
}

pub async fn execute(args: StatusArgs) -> Result<()> {
    let repo = Repository::open(".")?;

    // Check for pending operations
    if let Some(op) = repo.storage().current_operation()? {
        output::warn(&format!("Operation in progress: {}", op.name()));
        output::hint("Run 'gt continue' or 'gt abort'");
        println!();
    }

    // Show current branch info
    let current = match repo.current_branch()? {
        Some(b) => b,
        None => {
            output::warn("Not on a branch (detached HEAD)");
            return Ok(());
        }
    };

    println!("{}", "Current Branch".bold());
    println!("  {} {}", output::ARROW, output::branch(&current, true));

    // Check if tracked
    if !repo.storage().is_tracked(&current) {
        println!("  {} Not tracked by Stack", "!".yellow());
        output::hint(&format!("Run 'gt track' to start tracking '{}'", current));
        println!();

        // Still show working tree status
        show_working_tree_status(&repo)?;
        return Ok(());
    }

    // Get branch info
    let info = repo.storage().load_branch(&current)?.unwrap();

    println!("  Parent: {}", info.parent);

    if let Some(ref base) = info.base_commit {
        println!("  Base: {}", &base[..7.min(base.len())]);
    }

    // Show MR info
    if let Some(mr_number) = info.merge_request_id {
        println!();
        println!("{}", "Merge Request".bold());
        println!("  {} #{}", output::ARROW, format!("{}", mr_number).cyan());

        if let Some(ref url) = info.merge_request_url {
            println!("  {}", url.dimmed());
        }

        // Fetch detailed MR info from provider if requested or verbose
        if args.fetch || args.verbose {
            if let Err(e) = show_mr_details(&repo, mr_number).await {
                output::warn(&format!("Could not fetch MR details: {}", e));
            }
        }
    } else {
        println!();
        println!("{}", "Merge Request".bold());
        println!("  {} No MR created", "!".yellow());
        output::hint("Run 'gt submit' to create an MR");
    }

    // Show stack position
    let graph = repo.load_graph()?;
    let stack = graph.stack(&current);

    if stack.len() > 1 {
        println!();
        println!("{}", "Stack Position".bold());

        let current_idx = stack.iter().position(|b| *b == current).unwrap_or(0);
        println!("  Position: {} of {}", current_idx + 1, stack.len());

        // Show immediate neighbors
        if current_idx > 0 {
            println!("  Below: {}", stack[current_idx - 1]);
        }
        if current_idx < stack.len() - 1 {
            println!("  Above: {}", stack[current_idx + 1]);
        }
    }

    // Show children
    let children = graph.children(&current);
    if !children.is_empty() {
        println!();
        println!("{}", "Children".bold());
        for child in children {
            let child_info = repo.storage().load_branch(child)?;
            let mr_str = child_info
                .and_then(|i| i.merge_request_id)
                .map(|n| format!(" #{}", n).cyan().to_string())
                .unwrap_or_default();
            println!("  {} {}{}", output::ARROW, child, mr_str);
        }
    }

    // Working tree status
    println!();
    show_working_tree_status(&repo)?;

    Ok(())
}

fn show_working_tree_status(repo: &Repository) -> Result<()> {
    println!("{}", "Working Tree".bold());

    let statuses = repo.git().statuses(None)?;

    if statuses.is_empty() {
        println!("  {} Clean", "✓".green());
    } else {
        let mut staged = 0;
        let mut modified = 0;
        let mut untracked = 0;

        for entry in statuses.iter() {
            let status = entry.status();
            if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() {
                staged += 1;
            }
            if status.is_wt_modified() || status.is_wt_deleted() {
                modified += 1;
            }
            if status.is_wt_new() {
                untracked += 1;
            }
        }

        if staged > 0 {
            println!("  {} staged", format!("{} file(s)", staged).green());
        }
        if modified > 0 {
            println!("  {} modified", format!("{} file(s)", modified).yellow());
        }
        if untracked > 0 {
            println!("  {} untracked", format!("{} file(s)", untracked).dimmed());
        }
    }

    Ok(())
}

async fn show_mr_details(repo: &Repository, mr_number: u64) -> Result<()> {
    let ctx = ProviderContext::from_repo(repo).await?;

    let mr = ctx.provider().get_mr(&ctx.repo_id, mr_number.into()).await?;

    // State
    let state_str = match mr.state {
        stack_provider_api::MergeRequestState::Open => "Open".green(),
        stack_provider_api::MergeRequestState::Closed => "Closed".red(),
        stack_provider_api::MergeRequestState::Merged => "Merged".purple(),
        stack_provider_api::MergeRequestState::Draft => "Draft".yellow(),
    };
    println!("  State: {}{}", state_str, if mr.is_draft { " (draft)".dimmed().to_string() } else { String::new() });

    // Mergeable
    if let Some(mergeable) = mr.mergeable {
        let merge_str = if mergeable {
            "Yes".green()
        } else {
            "No".red()
        };
        println!("  Mergeable: {}", merge_str);
    }

    // Labels
    if !mr.labels.is_empty() {
        println!("  Labels: {}", mr.labels.join(", "));
    }

    // Provider
    println!("  Provider: {}", ctx.provider_type);

    Ok(())
}
