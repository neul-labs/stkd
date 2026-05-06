//! Stack Core - Stacked diffs for Git
//!
//! This library provides the core functionality for managing stacked branches
//! (stacked diffs/PRs) in Git repositories. It handles:
//!
//! - Branch tracking and dependency management
//! - Stack operations (create, modify, restack)
//! - Metadata persistence
//! - Rebase automation
//!
//! # Example
//!
//! ```rust,ignore
//! use stkd_core::{Repository, StackConfig};
//!
//! let repo = Repository::open(".")?;
//! let stack = repo.get_current_stack()?;
//!
//! // Create a new branch on top of current
//! repo.create_branch("feature/auth")?;
//!
//! // View the stack
//! for branch in stack.branches() {
//!     println!("{}", branch.name());
//! }
//! ```

pub mod branch;
pub mod config;
pub mod dag;
pub mod error;
pub mod history;
pub mod rebase;
pub mod repository;
pub mod stack;
pub mod storage;
pub mod template;

pub use branch::{Branch, BranchInfo, BranchStatus};
pub use config::{StackConfig, SubmitConfig};
pub use dag::BranchGraph;
pub use error::{Error, Result};
pub use history::{History, HistoryEntry};
pub use repository::Repository;
pub use stack::{Stack, StackEntry};
pub use storage::{RepoLock, Storage};
pub use template::{StackTemplate, TemplateStore};
