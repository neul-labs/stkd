//! Error types for Stack

use thiserror::Error;

/// Result type alias for Stack operations
pub type Result<T> = std::result::Result<T, Error>;

/// Stack error types
#[derive(Error, Debug)]
pub enum Error {
    /// Not a Git repository
    #[error("Not a git repository (or any parent up to mount point)")]
    NotARepository,

    /// Stack not initialized in this repository
    #[error("Stack not initialized. Run 'gt init' first.")]
    NotInitialized,

    /// Branch not found
    #[error("Branch not found: {0}")]
    BranchNotFound(String),

    /// Branch not tracked by Stack
    #[error("Branch '{0}' is not tracked by Stack. Run 'gt track {0}' first.")]
    BranchNotTracked(String),

    /// Branch already exists
    #[error("Branch already exists: {0}")]
    BranchExists(String),

    /// Cannot operate on trunk
    #[error("Cannot perform this operation on trunk branch '{0}'")]
    CannotOperateOnTrunk(String),

    /// Cycle detected in branch graph
    #[error("Cycle detected in branch dependencies: {0}")]
    CycleDetected(String),

    /// Rebase conflict
    #[error("Rebase conflict in branch '{0}'. Resolve conflicts and run 'gt continue'.")]
    RebaseConflict(String),

    /// Operation in progress
    #[error("Another operation is in progress: {0}. Run 'gt continue' or 'gt abort'.")]
    OperationInProgress(String),

    /// No operation in progress
    #[error("No operation in progress")]
    NoOperationInProgress,

    /// Invalid branch name
    #[error("Invalid branch name: {0}")]
    InvalidBranchName(String),

    /// No parent branch
    #[error("Branch '{0}' has no parent (is it the trunk?)")]
    NoParent(String),

    /// Uncommitted changes
    #[error("You have uncommitted changes. Commit or stash them first.")]
    UncommittedChanges,

    /// PR not found
    #[error("No pull request found for branch '{0}'")]
    PrNotFound(String),

    /// Already has PR
    #[error("Branch '{0}' already has an associated PR: #{1}")]
    PrAlreadyExists(String, u64),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Git operation error
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Other error
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Error::Config(msg.into())
    }

    /// Create a storage error
    pub fn storage(msg: impl Into<String>) -> Self {
        Error::Storage(msg.into())
    }

    /// Create a generic error
    pub fn other(msg: impl Into<String>) -> Self {
        Error::Other(msg.into())
    }

    /// Check if this error is recoverable (user can take action)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Error::RebaseConflict(_)
                | Error::UncommittedChanges
                | Error::OperationInProgress(_)
                | Error::BranchNotTracked(_)
        )
    }
}
