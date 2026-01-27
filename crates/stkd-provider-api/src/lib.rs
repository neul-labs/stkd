//! # Stack Provider API
//!
//! This crate provides the provider abstraction layer for Stack, enabling
//! pluggable support for different Git hosting platforms (GitHub, GitLab, Gitea, etc.).
//!
//! ## Overview
//!
//! The provider API defines a set of traits that abstract common Git hosting operations:
//!
//! - **Merge Requests**: Create, update, merge, and close pull/merge requests
//! - **Pipelines**: Monitor CI/CD status and trigger pipelines
//! - **Reviews**: Request reviews and check approval status
//! - **Labels & Milestones**: Manage issue tracking metadata
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                      stkd-cli                           │
//! │  (Commands use Provider trait, provider-agnostic)       │
//! └──────────────────────────┬──────────────────────────────┘
//!                            │
//! ┌──────────────────────────▼──────────────────────────────┐
//! │                  stkd-provider-api                      │
//! │  (Traits: Provider, MergeRequestProvider, etc.)         │
//! └──────────────────────────┬──────────────────────────────┘
//!                            │
//!          ┌─────────────────┼─────────────────┐
//!          │                 │                 │
//!    ┌─────▼─────┐    ┌─────▼─────┐    ┌─────▼─────┐
//!    │  GitHub   │    │  GitLab   │    │   Gitea   │
//!    │ Provider  │    │ Provider  │    │ Provider  │
//!    └───────────┘    └───────────┘    └───────────┘
//! ```
//!
//! ## Usage
//!
//! ### Implementing a Provider
//!
//! To add support for a new Git hosting platform:
//!
//! 1. Implement the required traits: [`MergeRequestProvider`], [`UserProvider`], [`RepositoryProvider`]
//! 2. Implement optional traits for additional features
//! 3. Implement the [`Provider`] trait
//!
//! ```rust,ignore
//! use stkd_provider_api::*;
//! use async_trait::async_trait;
//!
//! pub struct MyProvider {
//!     // API client, configuration, etc.
//! }
//!
//! #[async_trait]
//! impl MergeRequestProvider for MyProvider {
//!     async fn create_mr(&self, repo: &RepoId, request: CreateMergeRequest)
//!         -> ProviderResult<MergeRequest>
//!     {
//!         // Implementation
//!     }
//!     // ... other methods
//! }
//!
//! // ... implement UserProvider, RepositoryProvider
//!
//! impl Provider for MyProvider {
//!     fn name(&self) -> &'static str { "myprovider" }
//!     fn display_name(&self) -> &'static str { "My Provider" }
//!     fn capabilities(&self) -> ProviderCapabilities {
//!         ProviderCapabilities {
//!             merge_requests: true,
//!             pipelines: false,
//!             // ...
//!         }
//!     }
//! }
//! ```
//!
//! ### Using a Provider
//!
//! ```rust,ignore
//! use stkd_provider_api::*;
//!
//! async fn create_pr(provider: &dyn Provider, repo: &RepoId) -> ProviderResult<MergeRequest> {
//!     let request = CreateMergeRequest {
//!         title: "Add feature X".to_string(),
//!         source_branch: "feature/x".to_string(),
//!         target_branch: "main".to_string(),
//!         ..Default::default()
//!     };
//!
//!     provider.create_mr(repo, request).await
//! }
//! ```
//!
//! ## Feature Detection
//!
//! Not all providers support all features. Use [`Provider::capabilities()`] to
//! check what's available:
//!
//! ```rust,ignore
//! fn check_features(provider: &dyn Provider) {
//!     let caps = provider.capabilities();
//!
//!     if caps.pipelines {
//!         if let Some(pipeline_provider) = provider.pipelines() {
//!             // Use pipeline features
//!         }
//!     }
//!
//!     if caps.labels {
//!         if let Some(label_provider) = provider.labels() {
//!             // Use label features
//!         }
//!     }
//! }
//! ```
//!
//! ## Authentication
//!
//! The [`auth`] module provides a unified interface for credential management:
//!
//! ```rust,ignore
//! use stkd_provider_api::auth::{Credential, StoredCredential, FileCredentialStore, CredentialStore};
//!
//! // Create a credential
//! let cred = StoredCredential::new(
//!     "github",
//!     "github.com",
//!     Credential::pat("ghp_xxxx"),
//! );
//!
//! // Save it
//! let store = FileCredentialStore::new()?;
//! store.save(&cred)?;
//!
//! // Load it later
//! let loaded = store.load("github", "github.com")?;
//! ```

pub mod auth;
pub mod error;
pub mod traits;
pub mod types;

// Re-export commonly used items at the crate root
pub use auth::{
    clear_credentials, load_credentials, save_credentials, Credential, CredentialStore,
    FileCredentialStore, StoredCredential,
};
pub use error::{ProviderError, ProviderResult};
pub use traits::{
    ApprovalProvider, BranchProtectionProvider, LabelProvider, MergeRequestProvider,
    MilestoneProvider, PipelineProvider, Provider, RepositoryProvider, UserProvider,
};
pub use types::{
    ApprovalState, BranchProtection, CreateMergeRequest, Label, MergeMethod, MergeRequest,
    MergeRequestFilter, MergeRequestId, MergeRequestState, MergeResult, Milestone, MilestoneState,
    Pipeline, PipelineJob, PipelineStatus, ProviderCapabilities, RepoId, Review,
    UpdateMergeRequest, User,
};
