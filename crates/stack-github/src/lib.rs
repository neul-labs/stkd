//! GitHub integration for Stack
//!
//! Provides GitHub API operations for:
//! - Authentication (OAuth, personal access tokens)
//! - Pull request creation and management
//! - Sync with remote repository state
//! - PR status (reviews, CI checks)

pub mod auth;
pub mod api;
pub mod pr;
pub mod sync;

pub use auth::{GitHubAuth, AuthToken};
pub use api::GitHubClient;
pub use pr::{PullRequest, PullRequestCreate, PullRequestUpdate};
pub use sync::RemoteSync;
