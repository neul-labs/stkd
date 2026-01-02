//! Storage layer for Stack metadata.
//!
//! This module provides persistent storage for Stack's internal state,
//! stored in the `.git/stack/` directory.
//!
//! # Directory Structure
//!
//! ```text
//! .git/stack/
//! ├── config.json           # Stack configuration (trunk, provider, etc.)
//! ├── state.json            # Current operation state
//! └── branches/
//!     ├── feature-a.json    # Metadata for 'feature-a' branch
//!     └── feature-b.json    # Metadata for 'feature-b' branch
//! ```
//!
//! # Usage
//!
//! The [`Storage`] struct is the main interface. It's typically accessed
//! through a [`Repository`](crate::Repository):
//!
//! ```rust,ignore
//! let repo = Repository::open(".")?;
//! let branches = repo.storage().list_branches()?;
//! ```

use std::path::{Path, PathBuf};
use std::fs;

use crate::branch::BranchInfo;
use crate::config::StackConfig;
use crate::{Error, Result};

/// Current state of a Stack-enabled repository.
///
/// This structure tracks the repository's operational state, including
/// any in-progress operations that can be continued or aborted.
/// Stored in `.git/stack/state.json`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StackState {
    /// Currently checked out branch (cached for performance).
    pub current_branch: Option<String>,

    /// Branches that need restacking after changes to their parents.
    #[serde(default)]
    pub pending_restack: Vec<String>,

    /// State of an in-progress rebase that encountered conflicts.
    /// When set, the user needs to resolve conflicts and run `gt continue`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_state: Option<ConflictState>,

    /// Timestamp of the last successful sync with the remote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,

    /// An operation that's currently in progress.
    /// Used to resume operations after conflicts or errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<OngoingOperation>,
}

impl Default for StackState {
    fn default() -> Self {
        Self {
            current_branch: None,
            pending_restack: vec![],
            conflict_state: None,
            last_sync: None,
            operation: None,
        }
    }
}

/// State saved when a rebase encounters conflicts.
///
/// When restacking or rebasing encounters a conflict, this state is saved
/// so the operation can be resumed after the user resolves conflicts.
///
/// # Workflow
///
/// 1. User runs `gt restack` or `gt sync`
/// 2. Conflict is detected during rebase
/// 3. `ConflictState` is saved with the current context
/// 4. User resolves conflicts and stages changes
/// 5. User runs `gt continue`
/// 6. Stack resumes from the saved state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConflictState {
    /// The branch currently being rebased.
    pub branch: String,
    /// The target branch we're rebasing onto.
    pub onto: String,
    /// The commit SHA before the rebase started (for recovery).
    pub original_commit: String,
    /// Branches still waiting to be restacked after this one completes.
    pub remaining: Vec<String>,
}

/// An ongoing operation that can be continued or aborted.
///
/// When a multi-step operation is interrupted (e.g., by conflicts or errors),
/// its state is saved here. The user can then run `gt continue` to resume
/// or `gt abort` to cancel.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OngoingOperation {
    /// Restacking branches to update their parent relationships.
    /// Tracks which branches still need processing and which are done.
    Restack {
        /// Branches that still need to be restacked.
        branches: Vec<String>,
        /// Branches that have been successfully restacked.
        completed: Vec<String>,
    },
    /// Syncing with the remote repository.
    /// Tracks branches that should be deleted after sync completes.
    Sync {
        /// Local branches to delete (their MRs were merged).
        branches_to_delete: Vec<String>,
    },
    /// Submitting branches as merge/pull requests.
    /// Tracks submission progress for multi-branch submits.
    Submit {
        /// Branches that still need to be submitted.
        branches: Vec<String>,
        /// Branches that have been successfully submitted.
        completed: Vec<String>,
    },
}

impl OngoingOperation {
    pub fn name(&self) -> &'static str {
        match self {
            OngoingOperation::Restack { .. } => "restack",
            OngoingOperation::Sync { .. } => "sync",
            OngoingOperation::Submit { .. } => "submit",
        }
    }
}

/// Storage interface for Stack metadata.
///
/// Provides methods to load and save Stack's persistent state, including
/// configuration, branch metadata, and operation state.
///
/// # Thread Safety
///
/// This struct is not thread-safe. File operations are not locked.
/// For concurrent access, use external synchronization.
pub struct Storage {
    /// Path to .git/stack directory
    stack_dir: PathBuf,
    /// Path to branches directory
    branches_dir: PathBuf,
}

impl Storage {
    /// Open storage for a repository
    pub fn open(git_dir: &Path) -> Result<Self> {
        let stack_dir = git_dir.join("stack");
        let branches_dir = stack_dir.join("branches");

        Ok(Self {
            stack_dir,
            branches_dir,
        })
    }

    /// Initialize storage (create directories)
    pub fn init(git_dir: &Path) -> Result<Self> {
        let storage = Self::open(git_dir)?;

        // Create directories
        fs::create_dir_all(&storage.branches_dir)?;

        // Create default state
        if !storage.state_path().exists() {
            storage.save_state(&StackState::default())?;
        }

        Ok(storage)
    }

    /// Check if storage is initialized
    pub fn is_initialized(&self) -> bool {
        self.stack_dir.exists() && self.branches_dir.exists()
    }

    // ========================================================================
    // Config
    // ========================================================================

    fn config_path(&self) -> PathBuf {
        self.stack_dir.join("config.json")
    }

    /// Load configuration
    ///
    /// If the config is an older version, it will be automatically migrated
    /// to the current version and saved back to disk.
    pub fn load_config(&self) -> Result<StackConfig> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(StackConfig::default());
        }

        let content = fs::read_to_string(&path)?;
        let mut config: StackConfig = serde_json::from_str(&content)?;

        // Check if migration is needed (v1 -> v2)
        if config.version < crate::config::CONFIG_VERSION {
            config.migrate();
            // Save the migrated config back to disk
            self.save_config(&config)?;
        }

        Ok(config)
    }

    /// Save configuration
    pub fn save_config(&self, config: &StackConfig) -> Result<()> {
        let path = self.config_path();
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&path, content)?;
        Ok(())
    }

    // ========================================================================
    // State
    // ========================================================================

    fn state_path(&self) -> PathBuf {
        self.stack_dir.join("state.json")
    }

    /// Load current state
    pub fn load_state(&self) -> Result<StackState> {
        let path = self.state_path();
        if !path.exists() {
            return Ok(StackState::default());
        }

        let content = fs::read_to_string(&path)?;
        let state: StackState = serde_json::from_str(&content)?;
        Ok(state)
    }

    /// Save current state
    pub fn save_state(&self, state: &StackState) -> Result<()> {
        let path = self.state_path();
        let content = serde_json::to_string_pretty(state)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Update state with a modifier function
    pub fn update_state<F>(&self, f: F) -> Result<StackState>
    where
        F: FnOnce(&mut StackState),
    {
        let mut state = self.load_state()?;
        f(&mut state);
        self.save_state(&state)?;
        Ok(state)
    }

    // ========================================================================
    // Branches
    // ========================================================================

    fn branch_path(&self, name: &str) -> PathBuf {
        // Replace slashes in branch names with double underscores
        let safe_name = name.replace('/', "__");
        self.branches_dir.join(format!("{}.json", safe_name))
    }

    /// Load branch info
    pub fn load_branch(&self, name: &str) -> Result<Option<BranchInfo>> {
        let path = self.branch_path(name);
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        let info: BranchInfo = serde_json::from_str(&content)?;
        Ok(Some(info))
    }

    /// Save branch info
    pub fn save_branch(&self, info: &BranchInfo) -> Result<()> {
        let path = self.branch_path(&info.name);
        let content = serde_json::to_string_pretty(info)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Delete branch info
    pub fn delete_branch(&self, name: &str) -> Result<()> {
        let path = self.branch_path(name);
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }

    /// List all tracked branches
    pub fn list_branches(&self) -> Result<Vec<BranchInfo>> {
        let mut branches = vec![];

        if !self.branches_dir.exists() {
            return Ok(branches);
        }

        for entry in fs::read_dir(&self.branches_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "json") {
                let content = fs::read_to_string(&path)?;
                if let Ok(info) = serde_json::from_str::<BranchInfo>(&content) {
                    branches.push(info);
                }
            }
        }

        Ok(branches)
    }

    /// Check if a branch is tracked
    pub fn is_tracked(&self, name: &str) -> bool {
        self.branch_path(name).exists()
    }

    /// Update branch with a modifier function
    pub fn update_branch<F>(&self, name: &str, f: F) -> Result<BranchInfo>
    where
        F: FnOnce(&mut BranchInfo),
    {
        let mut info = self
            .load_branch(name)?
            .ok_or_else(|| Error::BranchNotTracked(name.to_string()))?;

        f(&mut info);
        info.touch();
        self.save_branch(&info)?;
        Ok(info)
    }

    // ========================================================================
    // Operations
    // ========================================================================

    /// Start an operation
    pub fn start_operation(&self, op: OngoingOperation) -> Result<()> {
        self.update_state(|state| {
            state.operation = Some(op);
        })?;
        Ok(())
    }

    /// Complete the current operation
    pub fn complete_operation(&self) -> Result<()> {
        self.update_state(|state| {
            state.operation = None;
            state.conflict_state = None;
        })?;
        Ok(())
    }

    /// Get current operation
    pub fn current_operation(&self) -> Result<Option<OngoingOperation>> {
        Ok(self.load_state()?.operation)
    }

    /// Set conflict state
    pub fn set_conflict(&self, conflict: ConflictState) -> Result<()> {
        self.update_state(|state| {
            state.conflict_state = Some(conflict);
        })?;
        Ok(())
    }

    /// Clear conflict state
    pub fn clear_conflict(&self) -> Result<()> {
        self.update_state(|state| {
            state.conflict_state = None;
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup() -> (TempDir, Storage) {
        let dir = TempDir::new().unwrap();
        let git_dir = dir.path().join(".git");
        fs::create_dir_all(&git_dir).unwrap();
        let storage = Storage::init(&git_dir).unwrap();
        (dir, storage)
    }

    #[test]
    fn test_storage_init() {
        let (_dir, storage) = setup();
        assert!(storage.is_initialized());
    }

    #[test]
    fn test_config_roundtrip() {
        let (_dir, storage) = setup();

        let mut config = StackConfig::default();
        config.trunk = "develop".to_string();

        storage.save_config(&config).unwrap();
        let loaded = storage.load_config().unwrap();

        assert_eq!(loaded.trunk, "develop");
    }

    #[test]
    fn test_branch_roundtrip() {
        let (_dir, storage) = setup();

        let info = BranchInfo::new("feature/test", "main");
        storage.save_branch(&info).unwrap();

        let loaded = storage.load_branch("feature/test").unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "feature/test");
    }

    #[test]
    fn test_branch_with_slashes() {
        let (_dir, storage) = setup();

        let info = BranchInfo::new("feature/auth/oauth", "main");
        storage.save_branch(&info).unwrap();

        let loaded = storage.load_branch("feature/auth/oauth").unwrap();
        assert!(loaded.is_some());
    }

    #[test]
    fn test_list_branches() {
        let (_dir, storage) = setup();

        storage.save_branch(&BranchInfo::new("feature/a", "main")).unwrap();
        storage.save_branch(&BranchInfo::new("feature/b", "main")).unwrap();

        let branches = storage.list_branches().unwrap();
        assert_eq!(branches.len(), 2);
    }

    #[test]
    fn test_state_operations() {
        let (_dir, storage) = setup();

        // Start operation
        storage.start_operation(OngoingOperation::Restack {
            branches: vec!["a".to_string(), "b".to_string()],
            completed: vec![],
        }).unwrap();

        let op = storage.current_operation().unwrap();
        assert!(op.is_some());
        assert_eq!(op.unwrap().name(), "restack");

        // Complete operation
        storage.complete_operation().unwrap();
        assert!(storage.current_operation().unwrap().is_none());
    }

    #[test]
    fn test_config_auto_migration() {
        let (_dir, storage) = setup();

        // Write a v1 config directly (simulating old format)
        let v1_config = r#"{
            "version": 1,
            "trunk": "main",
            "remote": "origin",
            "github": {
                "owner": "testowner",
                "repo": "testrepo"
            }
        }"#;

        fs::write(storage.config_path(), v1_config).unwrap();

        // Load config - should trigger automatic migration
        let loaded = storage.load_config().unwrap();

        // Verify migration happened
        assert_eq!(loaded.version, crate::config::CONFIG_VERSION);
        assert!(loaded.provider.is_some());
        assert!(loaded.github.is_none()); // Should be consumed

        let provider = loaded.provider.unwrap();
        assert_eq!(provider.owner, Some("testowner".to_string()));
        assert_eq!(provider.repo, Some("testrepo".to_string()));
        assert_eq!(provider.provider_type, crate::config::ProviderType::GitHub);

        // Verify migration was saved to disk
        let content = fs::read_to_string(storage.config_path()).unwrap();
        let saved: StackConfig = serde_json::from_str(&content).unwrap();
        assert_eq!(saved.version, crate::config::CONFIG_VERSION);
        assert!(saved.provider.is_some());
    }
}
