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

pub mod error;
pub mod config;
pub mod branch;
pub mod stack;
pub mod dag;
pub mod rebase;
pub mod storage;
pub mod repository;
pub mod history;
pub mod template;

pub use error::{Error, Result};
pub use config::{StackConfig, SubmitConfig};
pub use branch::{Branch, BranchInfo, BranchStatus};
pub use stack::{Stack, StackEntry};
pub use dag::BranchGraph;
pub use storage::{Storage, RepoLock};
pub use repository::Repository;
pub use history::{History, HistoryEntry};
pub use template::{StackTemplate, TemplateStore};
