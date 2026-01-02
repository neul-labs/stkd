//! Redo command - redo the last undone Stack operation

use anyhow::Result;
use clap::Args;
use stack_core::{History, HistoryEntry, Repository};

use crate::output;

#[derive(Args)]
pub struct RedoArgs {
    /// Show what would be redone without actually doing it
    #[arg(long)]
    dry_run: bool,
}

pub async fn execute(args: RedoArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let stack_dir = repo.git().path().join("stack");
    let mut history = History::load(&stack_dir);

    // Check if there's anything to redo
    let entry = match history.peek_redo() {
        Some(e) => e.clone(),
        None => {
            output::warn("Nothing to redo");
            return Ok(());
        }
    };

    // Dry run mode
    if args.dry_run {
        output::info("Dry run - would redo:");
        output::info(&format!("  {}", entry.description()));
        output::hint("Run without --dry-run to execute");
        return Ok(());
    }

    // Perform the redo
    output::info(&format!("Redoing: {}", entry.description()));

    match &entry {
        HistoryEntry::CreateBranch { branch, parent, .. } => {
            // Redo create = restore the branch metadata
            let info = stack_core::BranchInfo::new(branch, parent);
            repo.storage().save_branch(&info)?;
            output::success(&format!("Restored tracking for '{}'", branch));
        }

        HistoryEntry::DeleteBranch { branch, .. } => {
            // Redo delete = remove the branch metadata again
            repo.storage().delete_branch(branch)?;
            output::success(&format!("Removed tracking for '{}'", branch));
        }

        HistoryEntry::RenameBranch { old_name, new_name, .. } => {
            // Redo rename = rename forward again
            if let Some(mut info) = repo.storage().load_branch(old_name)? {
                repo.storage().delete_branch(old_name)?;
                info.name = new_name.clone();
                repo.storage().save_branch(&info)?;
                output::success(&format!("Renamed '{}' to '{}'", old_name, new_name));
            }
        }

        HistoryEntry::Reparent {
            branch, new_parent, ..
        } => {
            // Redo reparent = apply new parent again
            repo.storage().update_branch(branch, |info| {
                info.parent = new_parent.clone();
            })?;
            output::success(&format!("Changed parent of '{}' to '{}'", branch, new_parent));
        }

        HistoryEntry::SetMergeRequest {
            branch, new_mr_id, ..
        } => {
            // Redo MR association = apply new MR ID again
            repo.storage().update_branch(branch, |info| {
                info.merge_request_id = Some(*new_mr_id);
            })?;
            output::success(&format!("Associated MR #{} with '{}'", new_mr_id, branch));
        }
    }

    // Move entry back to undo stack
    let entry = history.pop_for_redo().unwrap();
    history.push_to_undo(entry);
    history.save(&stack_dir)?;

    if history.can_redo() {
        output::hint("Run 'gt redo' again to redo more operations");
    }

    Ok(())
}
