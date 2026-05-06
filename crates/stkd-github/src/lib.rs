//! GitHub integration for Stack
//!
//! This crate provides GitHub API operations for the Stack CLI tool:
//!
//! - **Authentication**: OAuth device flow and personal access tokens
//! - **Pull requests**: Create, update, merge, and manage PRs
//! - **Reviews**: Request and list code reviews
//! - **CI/CD**: Check GitHub Actions status
//! - **Labels and Milestones**: Manage PR metadata
//!
//! # Provider API
//!
//! The [`GitHubProvider`] struct implements the provider traits from
//! `stkd-provider-api`, enabling pluggable support for multiple Git
//! hosting platforms.
//!
//! ```rust,ignore
//! use stkd_github::GitHubProvider;
//! use stkd_provider_api::{Provider, RepoId, MergeRequestProvider};
//!
//! let provider = GitHubProvider::new("ghp_your_token")?;
//! let repo = RepoId::new("owner", "repo");
//!
//! // Create a pull request
//! let mr = provider.create_mr(&repo, CreateMergeRequest {
//!     title: "My feature".to_string(),
//!     source_branch: "feature".to_string(),
//!     target_branch: "main".to_string(),
//!     ..Default::default()
//! }).await?;
//! ```
//!
//! # Legacy API
//!
//! The original [`GitHubClient`] API is still available for backward
//! compatibility but new code should prefer [`GitHubProvider`].

pub mod api;
pub mod auth;
pub mod oauth;
pub mod pr;
pub mod provider;
pub mod repo;
pub mod sync;

// Provider API (recommended)
pub use provider::GitHubProvider;

// Re-export provider types for convenience
pub use stkd_provider_api::{
    ApprovalProvider, ApprovalState, CreateMergeRequest, Label, LabelProvider,
    MergeMethod as ProviderMergeMethod, MergeRequest, MergeRequestId, MergeRequestProvider,
    MergeRequestState, MergeResult as ProviderMergeResult, Milestone, MilestoneProvider, Pipeline,
    PipelineProvider, PipelineStatus, Provider, ProviderCapabilities, RepoId, RepositoryProvider,
    Review, UpdateMergeRequest, User, UserProvider,
};

// Legacy API (for backward compatibility)
pub use api::GitHubClient;
pub use auth::{AuthToken, GitHubAuth};
pub use oauth::DeviceFlow;
pub use pr::{MergeMethod, MergeResult, PullRequest, PullRequestCreate, PullRequestUpdate};
pub use repo::RepoInfo;
pub use sync::RemoteSync;
