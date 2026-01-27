//! Fold command - fold staged changes into a previous commit

use anyhow::{Context, Result};
use clap::Args;
use stkd_core::Repository;
use std::path::Path;
use std::process::Command;

use crate::output;

/// Fold staged changes into a previous commit
#[derive(Args)]
pub struct FoldArgs {
    /// Target commit to fold into (default: HEAD)
    #[arg(long, short)]
    into: Option<String>,

    /// Create a fixup commit instead of immediately folding
    #[arg(long)]
    fixup: bool,
}

pub async fn execute(args: FoldArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let workdir = repo.git().path().parent().unwrap_or(Path::new("."));

    // Check for staged changes
    let index = repo.git().index()?;
    let head_tree = repo.git().head()?.peel_to_commit()?.tree()?;
    let diff = repo
        .git()
        .diff_tree_to_index(Some(&head_tree), Some(&index), None)?;

    if diff.deltas().len() == 0 {
        output::warn("No staged changes to fold");
        output::hint("Stage changes with 'git add' first");
        return Ok(());
    }

    let target = args.into.as_deref().unwrap_or("HEAD");
    output::info(&format!(
        "Folding {} staged changes into {}",
        diff.deltas().len(),
        target
    ));

    if args.fixup {
        // Create a fixup commit
        let status = Command::new("git")
            .args(["commit", "--fixup", target])
            .current_dir(workdir)
            .status()
            .context("Failed to create fixup commit")?;

        if !status.success() {
            anyhow::bail!("Git commit --fixup failed");
        }

        output::success("Created fixup commit");
        output::hint("Run 'git rebase -i --autosquash' to apply the fixup");
    } else {
        // Immediately fold by creating fixup and autosquashing
        let status = Command::new("git")
            .args(["commit", "--fixup", target])
            .current_dir(workdir)
            .status()
            .context("Failed to create fixup commit")?;

        if !status.success() {
            anyhow::bail!("Git commit --fixup failed");
        }

        // Get the parent branch for rebase
        let branch_name = repo.current_branch()?.ok_or_else(|| {
            anyhow::anyhow!("Not on a branch")
        })?;
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

        // Run autosquash rebase
        let status = Command::new("git")
            .args([
                "rebase",
                "-i",
                "--autosquash",
                "--autostash",
                &parent,
            ])
            .env("GIT_SEQUENCE_EDITOR", "true") // Auto-accept the rebase todo
            .current_dir(workdir)
            .status()
            .context("Failed to run autosquash rebase")?;

        if !status.success() {
            output::warn("Rebase encountered conflicts");
            output::hint("Resolve conflicts and run 'gt continue'");
            return Ok(());
        }

        output::success(&format!("Folded changes into {}", target));
    }

    Ok(())
}
