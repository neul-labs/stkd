//! Squash command - combine commits in the current branch

use anyhow::{Context, Result};
use clap::Args;
use std::path::Path;
use std::process::Command;
use stkd_core::Repository;

use crate::output;

/// Squash commits in the current branch
#[derive(Args)]
pub struct SquashArgs {
    /// Squash all commits into one
    #[arg(long, short)]
    all: bool,

    /// Number of commits to squash (from HEAD)
    #[arg(long, short = 'n')]
    count: Option<usize>,

    /// New commit message (if not provided, opens editor)
    #[arg(long, short)]
    message: Option<String>,
}

pub async fn execute(args: SquashArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let workdir = repo.git().path().parent().unwrap_or(Path::new("."));
    let branch_name = repo
        .current_branch()?
        .ok_or_else(|| anyhow::anyhow!("Not on a branch"))?;

    // Get the parent branch to find the merge base
    let graph = repo.load_graph()?;
    let branch_info = repo.storage().load_branch(&branch_name)?;

    let parent = if let Some(info) = branch_info {
        if graph.is_trunk(&info.parent) {
            repo.trunk().to_string()
        } else {
            info.parent.clone()
        }
    } else {
        repo.trunk().to_string()
    };

    output::info(&format!(
        "Squashing commits in {} (parent: {})",
        branch_name, parent
    ));

    // Get the merge base
    let merge_base = repo
        .git()
        .merge_base(
            repo.git()
                .find_reference(&format!("refs/heads/{}", parent))?
                .peel_to_commit()?
                .id(),
            repo.git().head()?.peel_to_commit()?.id(),
        )
        .context("Failed to find merge base")?;

    // Count commits since merge base
    let mut revwalk = repo.git().revwalk()?;
    revwalk.push(repo.git().head()?.peel_to_commit()?.id())?;
    revwalk.hide(merge_base)?;
    let total_commits = revwalk.count();

    if total_commits == 0 {
        output::warn("No commits to squash");
        return Ok(());
    }

    let commits_to_squash = if args.all {
        total_commits
    } else {
        args.count.unwrap_or(total_commits).min(total_commits)
    };

    if commits_to_squash <= 1 {
        output::info("Only one commit, nothing to squash");
        return Ok(());
    }

    output::info(&format!("Squashing {} commits...", commits_to_squash));

    // Soft reset to merge base
    let status = Command::new("git")
        .args(["reset", "--soft", &format!("HEAD~{}", commits_to_squash)])
        .current_dir(workdir)
        .status()
        .context("Failed to reset commits")?;

    if !status.success() {
        anyhow::bail!("Git reset failed");
    }

    // Commit with new message or open editor
    let commit_args = if let Some(msg) = args.message {
        vec!["commit".to_string(), "-m".to_string(), msg]
    } else {
        vec!["commit".to_string()]
    };

    let status = Command::new("git")
        .args(&commit_args)
        .current_dir(workdir)
        .status()
        .context("Failed to create squashed commit")?;

    if !status.success() {
        anyhow::bail!("Git commit failed");
    }

    output::success(&format!("Squashed {} commits into one", commits_to_squash));

    Ok(())
}
