//! Operation history for undo/redo support.
//!
//! This module tracks branch operations so they can be undone and redone.
//! History is stored in `.git/stack/history.json`.
//!
//! # Supported Operations
//!
//! - Branch creation/deletion
//! - Branch renames
//! - Parent changes (reparenting)
//! - MR associations
//!
//! # Limitations
//!
//! - Git commits cannot be undone (only branch metadata)
//! - History is limited to the last 50 operations by default

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Maximum number of history entries to keep
const MAX_HISTORY_SIZE: usize = 50;

/// A recorded operation that can be undone
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HistoryEntry {
    /// Created a new branch
    CreateBranch {
        /// Name of the created branch
        branch: String,
        /// Parent branch at creation time
        parent: String,
        /// When this operation occurred
        timestamp: DateTime<Utc>,
    },

    /// Deleted a branch
    DeleteBranch {
        /// Name of the deleted branch
        branch: String,
        /// Parent branch at deletion time
        parent: String,
        /// Children at deletion time (for restoration)
        children: Vec<String>,
        /// MR ID if one was associated
        merge_request_id: Option<u64>,
        /// When this operation occurred
        timestamp: DateTime<Utc>,
    },

    /// Renamed a branch
    RenameBranch {
        /// Old branch name
        old_name: String,
        /// New branch name
        new_name: String,
        /// When this operation occurred
        timestamp: DateTime<Utc>,
    },

    /// Changed a branch's parent
    Reparent {
        /// Branch that was reparented
        branch: String,
        /// Previous parent
        old_parent: String,
        /// New parent
        new_parent: String,
        /// When this operation occurred
        timestamp: DateTime<Utc>,
    },

    /// Associated an MR with a branch
    SetMergeRequest {
        /// Branch name
        branch: String,
        /// Previous MR ID (if any)
        old_mr_id: Option<u64>,
        /// New MR ID
        new_mr_id: u64,
        /// When this operation occurred
        timestamp: DateTime<Utc>,
    },
}

impl HistoryEntry {
    /// Get a human-readable description of this operation
    pub fn description(&self) -> String {
        match self {
            HistoryEntry::CreateBranch { branch, parent, .. } => {
                format!("Create branch '{}' on '{}'", branch, parent)
            }
            HistoryEntry::DeleteBranch { branch, .. } => {
                format!("Delete branch '{}'", branch)
            }
            HistoryEntry::RenameBranch { old_name, new_name, .. } => {
                format!("Rename '{}' to '{}'", old_name, new_name)
            }
            HistoryEntry::Reparent { branch, old_parent, new_parent, .. } => {
                format!("Move '{}' from '{}' to '{}'", branch, old_parent, new_parent)
            }
            HistoryEntry::SetMergeRequest { branch, new_mr_id, .. } => {
                format!("Associate MR #{} with '{}'", new_mr_id, branch)
            }
        }
    }

    /// Get the timestamp of this operation
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            HistoryEntry::CreateBranch { timestamp, .. }
            | HistoryEntry::DeleteBranch { timestamp, .. }
            | HistoryEntry::RenameBranch { timestamp, .. }
            | HistoryEntry::Reparent { timestamp, .. }
            | HistoryEntry::SetMergeRequest { timestamp, .. } => *timestamp,
        }
    }
}

/// Operation history tracker
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct History {
    /// Past operations (can be undone)
    pub undo_stack: Vec<HistoryEntry>,
    /// Undone operations (can be redone)
    pub redo_stack: Vec<HistoryEntry>,
}

impl History {
    /// Load history from disk
    pub fn load(stack_dir: &std::path::Path) -> Self {
        let path = Self::path(stack_dir);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(history) = serde_json::from_str(&content) {
                    return history;
                }
            }
        }
        Self::default()
    }

    /// Save history to disk
    pub fn save(&self, stack_dir: &std::path::Path) -> std::io::Result<()> {
        let path = Self::path(stack_dir);
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)
    }

    /// Get the path to the history file
    fn path(stack_dir: &std::path::Path) -> PathBuf {
        stack_dir.join("history.json")
    }

    /// Record a new operation
    pub fn record(&mut self, entry: HistoryEntry) {
        // Clear redo stack when a new operation is performed
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(entry);

        // Trim if over limit
        while self.undo_stack.len() > MAX_HISTORY_SIZE {
            self.undo_stack.remove(0);
        }
    }

    /// Pop the last operation for undoing
    pub fn pop_for_undo(&mut self) -> Option<HistoryEntry> {
        self.undo_stack.pop()
    }

    /// Push an entry to the redo stack after undoing
    pub fn push_to_redo(&mut self, entry: HistoryEntry) {
        self.redo_stack.push(entry);
    }

    /// Pop the last undone operation for redoing
    pub fn pop_for_redo(&mut self) -> Option<HistoryEntry> {
        self.redo_stack.pop()
    }

    /// Push an entry back to undo stack after redoing
    pub fn push_to_undo(&mut self, entry: HistoryEntry) {
        self.undo_stack.push(entry);
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get the last operation (for showing what will be undone)
    pub fn peek_undo(&self) -> Option<&HistoryEntry> {
        self.undo_stack.last()
    }

    /// Get the last undone operation (for showing what will be redone)
    pub fn peek_redo(&self) -> Option<&HistoryEntry> {
        self.redo_stack.last()
    }

    /// Get recent history entries for display
    pub fn recent(&self, count: usize) -> Vec<&HistoryEntry> {
        self.undo_stack.iter().rev().take(count).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_record_and_undo() {
        let mut history = History::default();

        // Record some operations
        history.record(HistoryEntry::CreateBranch {
            branch: "feature-a".to_string(),
            parent: "main".to_string(),
            timestamp: Utc::now(),
        });

        history.record(HistoryEntry::CreateBranch {
            branch: "feature-b".to_string(),
            parent: "feature-a".to_string(),
            timestamp: Utc::now(),
        });

        assert!(history.can_undo());
        assert!(!history.can_redo());

        // Undo last operation
        let entry = history.pop_for_undo().unwrap();
        history.push_to_redo(entry);

        assert!(history.can_undo());
        assert!(history.can_redo());

        // Redo
        let entry = history.pop_for_redo().unwrap();
        history.push_to_undo(entry);

        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_new_operation_clears_redo() {
        let mut history = History::default();

        history.record(HistoryEntry::CreateBranch {
            branch: "a".to_string(),
            parent: "main".to_string(),
            timestamp: Utc::now(),
        });

        // Undo
        let entry = history.pop_for_undo().unwrap();
        history.push_to_redo(entry);
        assert!(history.can_redo());

        // Record new operation
        history.record(HistoryEntry::CreateBranch {
            branch: "b".to_string(),
            parent: "main".to_string(),
            timestamp: Utc::now(),
        });

        // Redo should be cleared
        assert!(!history.can_redo());
    }
}
