//! GitLab integration for Stack
//!
//! This crate provides GitLab API operations for the Stack CLI tool:
//!
//! - **Authentication**: Personal access tokens, OAuth, job tokens
//! - **Merge Requests**: Create, update, merge, and manage MRs
//! - **Pipelines**: Check GitLab CI pipeline status
//! - **Approvals**: Request and track approvals
//! - **Labels and Milestones**: Manage MR metadata
//!
//! # Provider API
//!
//! The [`GitLabProvider`] struct implements the provider traits from
//! `stkd-provider-api`, enabling pluggable support for multiple Git
//! hosting platforms.
//!
//! ```rust,ignore
//! use stkd_gitlab::GitLabProvider;
//! use stkd_provider_api::{Provider, RepoId, MergeRequestProvider};
//!
//! let provider = GitLabProvider::new("glpat-your_token")?;
//! let repo = RepoId::new("group", "project");
//!
//! // Create a merge request
//! let mr = provider.create_mr(&repo, CreateMergeRequest {
//!     title: "My feature".to_string(),
//!     source_branch: "feature".to_string(),
//!     target_branch: "main".to_string(),
//!     ..Default::default()
//! }).await?;
//! ```
//!
//! # Self-Hosted GitLab
//!
//! For self-hosted GitLab instances, use the `with_host` constructor:
//!
//! ```rust,ignore
//! let provider = GitLabProvider::with_host("glpat-xxx", "gitlab.mycompany.com")?;
//! ```

pub mod auth;
pub mod provider;

// Provider API (recommended)
pub use provider::GitLabProvider;

// Re-export provider types for convenience
pub use stkd_provider_api::{
    ApprovalProvider, ApprovalState, CreateMergeRequest, Label, LabelProvider, MergeMethod,
    MergeRequest, MergeRequestId, MergeRequestProvider, MergeRequestState, Milestone,
    MilestoneProvider, Pipeline, PipelineProvider, PipelineStatus, Provider, ProviderCapabilities,
    ProviderError, ProviderResult, RepoId, RepositoryProvider, Review, UpdateMergeRequest, User,
    UserProvider,
};

// Auth types
pub use auth::{AuthToken, GitLabAuth};
