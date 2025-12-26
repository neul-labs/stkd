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
//! `stack-provider-api`, enabling pluggable support for multiple Git
//! hosting platforms.
//!
//! ```rust,ignore
//! use stack_github::GitHubProvider;
//! use stack_provider_api::{Provider, RepoId, MergeRequestProvider};
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

pub mod auth;
pub mod api;
pub mod oauth;
pub mod pr;
pub mod provider;
pub mod repo;
pub mod sync;

// Provider API (recommended)
pub use provider::GitHubProvider;

// Re-export provider types for convenience
pub use stack_provider_api::{
    Provider, MergeRequestProvider, UserProvider, RepositoryProvider,
    PipelineProvider, ApprovalProvider, LabelProvider, MilestoneProvider,
    RepoId, MergeRequestId, MergeRequest, CreateMergeRequest, UpdateMergeRequest,
    MergeMethod as ProviderMergeMethod, MergeResult as ProviderMergeResult,
    User, Review, Label, Milestone, Pipeline, PipelineStatus,
    ProviderCapabilities, MergeRequestState, ApprovalState,
};

// Legacy API (for backward compatibility)
pub use auth::{GitHubAuth, AuthToken};
pub use api::GitHubClient;
pub use oauth::DeviceFlow;
pub use pr::{PullRequest, PullRequestCreate, PullRequestUpdate, MergeMethod, MergeResult};
pub use repo::RepoInfo;
pub use sync::RemoteSync;
