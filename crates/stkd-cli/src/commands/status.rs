//! Status command - show current stack status

use anyhow::Result;
use clap::Args;
use colored::Colorize;
use serde::Serialize;
use stkd_core::Repository;

use crate::output;

#[derive(Args)]
pub struct StatusArgs {
    /// Show detailed MR information
    #[arg(long, short)]
    verbose: bool,

    /// Fetch latest status from provider
    #[arg(long)]
    fetch: bool,
}

#[derive(Serialize)]
struct StatusJson {
    current_branch: String,
    tracked: bool,
    parent: Option<String>,
    mr: Option<MrJson>,
    stack_position: Option<StackPositionJson>,
    children: Vec<ChildJson>,
    working_tree: WorkingTreeJson,
}

#[derive(Serialize)]
struct MrJson {
    number: u64,
    url: Option<String>,
    state: String,
    mergeable: Option<bool>,
    labels: Vec<String>,
}

#[derive(Serialize)]
struct StackPositionJson {
    position: usize,
    total: usize,
    below: Option<String>,
    above: Option<String>,
}

#[derive(Serialize)]
struct ChildJson {
    name: String,
    mr_number: Option<u64>,
}

#[derive(Serialize)]
struct WorkingTreeJson {
    clean: bool,
    staged: usize,
    modified: usize,
    untracked: usize,
}

pub async fn execute(args: StatusArgs, json: bool) -> Result<()> {
    let repo = Repository::open(".")?;

    let current = match repo.current_branch()? {
        Some(b) => b,
        None => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &serde_json::json!({ "error": "Not on a branch (detached HEAD)" })
                    )?
                );
            } else {
                output::warn("Not on a branch (detached HEAD)");
            }
            return Ok(());
        }
    };

    let tracked = repo.storage().is_tracked(&current);
    let info = if tracked {
        repo.storage().load_branch(&current)?
    } else {
        None
    };

    let graph = repo.load_graph()?;
    let stack = graph.stack(&current);

    let mut mr_json = None;
    if let Some(ref info) = info {
        if let Some(mr_number) = info.merge_request_id {
            let mut mr_state = "unknown".to_string();
            let mut mergeable = None;
            let mut labels = Vec::new();

            if args.fetch || args.verbose {
                if let Ok(ctx) = stkd_engine::ProviderContext::from_repo(&repo).await {
                    if let Ok(mr) = ctx.provider().get_mr(&ctx.repo_id, mr_number.into()).await {
                        mr_state = format!("{:?}", mr.state);
                        mergeable = mr.mergeable;
                        labels = mr.labels;
                    }
                }
            }

            mr_json = Some(MrJson {
                number: mr_number,
                url: info.merge_request_url.clone(),
                state: mr_state,
                mergeable,
                labels,
            });
        }
    }

    let stack_position = if stack.len() > 1 {
        let current_idx = stack.iter().position(|b| *b == current).unwrap_or(0);
        Some(StackPositionJson {
            position: current_idx + 1,
            total: stack.len(),
            below: if current_idx > 0 {
                Some(stack[current_idx - 1].to_string())
            } else {
                None
            },
            above: if current_idx < stack.len() - 1 {
                Some(stack[current_idx + 1].to_string())
            } else {
                None
            },
        })
    } else {
        None
    };

    let children: Vec<ChildJson> = graph
        .children(&current)
        .into_iter()
        .map(|child| {
            let child_info = repo.storage().load_branch(child).ok().flatten();
            ChildJson {
                name: child.to_string(),
                mr_number: child_info.as_ref().and_then(|i| i.merge_request_id),
            }
        })
        .collect();

    let (wt_clean, wt_staged, wt_modified, wt_untracked) = get_working_tree_status(&repo)?;

    if json {
        let status = StatusJson {
            current_branch: current.clone(),
            tracked,
            parent: info.as_ref().map(|i| i.parent.clone()),
            mr: mr_json,
            stack_position,
            children,
            working_tree: WorkingTreeJson {
                clean: wt_clean,
                staged: wt_staged,
                modified: wt_modified,
                untracked: wt_untracked,
            },
        };
        println!("{}", serde_json::to_string_pretty(&status)?);
        return Ok(());
    }

    // Human output
    println!("{}", "Current Branch".bold());
    println!("  {} {}", output::ARROW, output::branch(&current, true));

    if !tracked {
        println!("  {} Not tracked by Stack", "!".yellow());
        output::hint(&format!("Run 'gt track' to start tracking '{}'", current));
        println!();
        show_working_tree_status_human(wt_clean, wt_staged, wt_modified, wt_untracked)?;
        return Ok(());
    }

    if let Some(ref info) = info {
        println!("  Parent: {}", info.parent);
        if let Some(ref base) = info.base_commit {
            println!("  Base: {}", &base[..7.min(base.len())]);
        }
    }

    if let Some(ref mr) = mr_json {
        println!();
        println!("{}", "Merge Request".bold());
        println!("  {} #{}", output::ARROW, mr.number);
        if let Some(ref url) = mr.url {
            println!("  {}", url.dimmed());
        }
        println!("  State: {}", mr.state);
        if let Some(m) = mr.mergeable {
            println!(
                "  Mergeable: {}",
                if m { "Yes".green() } else { "No".red() }
            );
        }
        if !mr.labels.is_empty() {
            println!("  Labels: {}", mr.labels.join(", "));
        }
    } else {
        println!();
        println!("{}", "Merge Request".bold());
        println!("  {} No MR created", "!".yellow());
        output::hint("Run 'gt submit' to create an MR");
    }

    if let Some(ref pos) = stack_position {
        println!();
        println!("{}", "Stack Position".bold());
        println!("  Position: {} of {}", pos.position, pos.total);
        if let Some(ref below) = pos.below {
            println!("  Below: {}", below);
        }
        if let Some(ref above) = pos.above {
            println!("  Above: {}", above);
        }
    }

    if !children.is_empty() {
        println!();
        println!("{}", "Children".bold());
        for child in &children {
            let mr_str = child
                .mr_number
                .map(|n| format!(" #{}", n).cyan().to_string())
                .unwrap_or_default();
            println!("  {} {}{}", output::ARROW, child.name, mr_str);
        }
    }

    println!();
    show_working_tree_status_human(wt_clean, wt_staged, wt_modified, wt_untracked)?;

    Ok(())
}

fn get_working_tree_status(repo: &Repository) -> Result<(bool, usize, usize, usize)> {
    let statuses = repo.git().statuses(None)?;

    if statuses.is_empty() {
        return Ok((true, 0, 0, 0));
    }

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

    Ok((false, staged, modified, untracked))
}

fn show_working_tree_status_human(
    clean: bool,
    staged: usize,
    modified: usize,
    untracked: usize,
) -> Result<()> {
    println!("{}", "Working Tree".bold());

    if clean {
        println!("  {} Clean", "✓".green());
    } else {
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
