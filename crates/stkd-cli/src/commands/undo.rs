//! Undo command - undo the last Stack operation

use anyhow::Result;
use clap::Args;
use stkd_core::{History, HistoryEntry, Repository};

use crate::output;

#[derive(Args)]
pub struct UndoArgs {
    /// Show what would be undone without actually doing it
    #[arg(long)]
    dry_run: bool,

    /// Show recent operations history
    #[arg(long)]
    list: bool,
}

pub async fn execute(args: UndoArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let stack_dir = repo.git().path().join("stkd");
    let mut history = History::load(&stack_dir);

    // List mode - show recent history
    if args.list {
        let recent = history.recent(10);
        if recent.is_empty() {
            output::info("No operations in history");
        } else {
            output::info("Recent operations (newest first):");
            output::info("");
            for (i, entry) in recent.iter().enumerate() {
                let marker = if i == 0 { "→" } else { " " };
                output::info(&format!(
                    "  {} {} ({})",
                    marker,
                    entry.description(),
                    entry.timestamp().format("%Y-%m-%d %H:%M")
                ));
            }
            output::info("");
            if history.can_undo() {
                output::hint("Run 'gt undo' to undo the most recent operation");
            }
        }
        return Ok(());
    }

    // Check if there's anything to undo
    let entry = match history.peek_undo() {
        Some(e) => e.clone(),
        None => {
            output::warn("Nothing to undo");
            return Ok(());
        }
    };

    // Dry run mode
    if args.dry_run {
        output::info("Dry run - would undo:");
        output::info(&format!("  {}", entry.description()));
        output::hint("Run without --dry-run to execute");
        return Ok(());
    }

    // Perform the undo
    output::info(&format!("Undoing: {}", entry.description()));

    match &entry {
        HistoryEntry::CreateBranch { branch, .. } => {
            // Undo create = delete the branch metadata
            repo.storage().delete_branch(branch)?;
            output::success(&format!("Removed tracking for '{}'", branch));
            output::hint(&format!(
                "Note: The git branch still exists. Run 'git branch -d {}' to delete it.",
                branch
            ));
        }

        HistoryEntry::DeleteBranch {
            branch,
            parent,
            merge_request_id,
            ..
        } => {
            // Undo delete = restore the branch metadata
            let mut info = stkd_core::BranchInfo::new(branch, parent);
            if let Some(mr_id) = merge_request_id {
                info.merge_request_id = Some(*mr_id);
            }
            repo.storage().save_branch(&info)?;
            output::success(&format!("Restored tracking for '{}'", branch));
        }

        HistoryEntry::RenameBranch { old_name, new_name, .. } => {
            // Undo rename = rename back
            if let Some(mut info) = repo.storage().load_branch(new_name)? {
                repo.storage().delete_branch(new_name)?;
                info.name = old_name.clone();
                repo.storage().save_branch(&info)?;
                output::success(&format!("Renamed '{}' back to '{}'", new_name, old_name));
            }
        }

        HistoryEntry::Reparent {
            branch, old_parent, ..
        } => {
            // Undo reparent = restore old parent
            repo.storage().update_branch(branch, |info| {
                info.parent = old_parent.clone();
            })?;
            output::success(&format!("Restored parent of '{}' to '{}'", branch, old_parent));
        }

        HistoryEntry::SetMergeRequest {
            branch, old_mr_id, ..
        } => {
            // Undo MR association = restore old MR ID
            repo.storage().update_branch(branch, |info| {
                info.merge_request_id = *old_mr_id;
            })?;
            if let Some(old_id) = old_mr_id {
                output::success(&format!("Restored MR #{} for '{}'", old_id, branch));
            } else {
                output::success(&format!("Cleared MR association for '{}'", branch));
            }
        }
    }

    // Move entry to redo stack
    let entry = history.pop_for_undo().unwrap();
    history.push_to_redo(entry);
    history.save(&stack_dir)?;

    if history.can_undo() {
        output::hint("Run 'gt undo' again to undo more operations");
    }

    Ok(())
}
