//! Split command - split the current commit into multiple commits

use anyhow::{Context, Result};
use clap::Args;
use stkd_core::Repository;
use std::path::Path;
use std::process::Command;

use crate::output;

/// Split the current commit into multiple commits
#[derive(Args)]
pub struct SplitArgs {
    /// Number of commits to create (opens interactive add for each)
    #[arg(long, short, default_value = "2")]
    count: usize,
}

pub async fn execute(args: SplitArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let workdir = repo.git().path().parent().unwrap_or(Path::new("."));

    if args.count < 2 {
        anyhow::bail!("Cannot split into fewer than 2 commits");
    }

    // Get current commit info
    let head = repo.git().head()?.peel_to_commit()?;
    let commit_msg = head.message().unwrap_or("").to_string();

    output::info(&format!(
        "Splitting current commit into {} commits",
        args.count
    ));
    output::info(&format!("Original message: {}", commit_msg.lines().next().unwrap_or("")));

    // Soft reset HEAD~1 to unstage the commit
    let status = Command::new("git")
        .args(["reset", "--soft", "HEAD~1"])
        .current_dir(workdir)
        .status()
        .context("Failed to reset HEAD")?;

    if !status.success() {
        anyhow::bail!("Git reset failed");
    }

    // Unstage all files but keep working directory changes
    let status = Command::new("git")
        .args(["reset", "HEAD"])
        .current_dir(workdir)
        .status()
        .context("Failed to unstage files")?;

    if !status.success() {
        anyhow::bail!("Git unstage failed");
    }

    output::success("Commit undone, changes preserved in working directory");
    output::info("");
    output::info("Now interactively create your commits:");
    output::hint("Use 'git add -p' to stage parts of files");
    output::hint("Then 'git commit' to create each commit");
    output::hint("");

    // Interactive staging loop
    for i in 1..=args.count {
        output::info(&format!("--- Commit {}/{} ---", i, args.count));

        if i < args.count {
            // Open interactive staging
            let status = Command::new("git")
                .args(["add", "-p"])
                .current_dir(workdir)
                .status()
                .context("Failed to run git add -p")?;

            if !status.success() {
                output::warn("Interactive staging cancelled");
                continue;
            }

            // Check if anything was staged
            let index = repo.git().index()?;
            let head_tree = if let Ok(head) = repo.git().head() {
                if let Ok(commit) = head.peel_to_commit() {
                    Some(commit.tree()?)
                } else {
                    None
                }
            } else {
                None
            };

            let diff = if let Some(tree) = head_tree {
                repo.git().diff_tree_to_index(Some(&tree), Some(&index), None)?
            } else {
                repo.git().diff_tree_to_index(None, Some(&index), None)?
            };

            if diff.deltas().len() == 0 {
                output::warn("Nothing staged, skipping commit");
                continue;
            }

            // Create commit
            let status = Command::new("git")
                .args(["commit"])
                .current_dir(workdir)
                .status()
                .context("Failed to create commit")?;

            if !status.success() {
                output::warn("Commit cancelled");
            } else {
                output::success(&format!("Created commit {}/{}", i, args.count));
            }
        } else {
            // Last commit: commit remaining changes
            output::info("Committing remaining changes...");

            let status = Command::new("git")
                .args(["add", "-A"])
                .current_dir(workdir)
                .status()
                .context("Failed to stage remaining changes")?;

            if !status.success() {
                anyhow::bail!("Git add failed");
            }

            // Check if anything to commit
            let status_output = Command::new("git")
                .args(["status", "--porcelain"])
                .current_dir(workdir)
                .output()
                .context("Failed to check status")?;

            if status_output.stdout.is_empty() {
                output::info("No remaining changes");
            } else {
                let status = Command::new("git")
                    .args(["commit"])
                    .current_dir(workdir)
                    .status()
                    .context("Failed to create final commit")?;

                if status.success() {
                    output::success(&format!("Created commit {}/{}", i, args.count));
                }
            }
        }
    }

    output::success("Split complete!");
    output::hint("Use 'gt log' to see the new commits");

    Ok(())
}
