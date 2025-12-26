//! Provider traits for Git hosting platforms.
//!
//! This module defines the trait hierarchy that all Git hosting provider
//! implementations must satisfy. The traits are designed to be composable,
//! allowing providers to implement only the features they support.
//!
//! # Trait Hierarchy
//!
//! ```text
//! Provider (main trait)
//! ├── MergeRequestProvider (required)
//! ├── UserProvider (required)
//! ├── RepositoryProvider (required)
//! ├── PipelineProvider (optional)
//! ├── ApprovalProvider (optional)
//! ├── LabelProvider (optional)
//! └── MilestoneProvider (optional)
//! ```
//!
//! # Implementing a Provider
//!
//! To create a new provider:
//!
//! 1. Implement the required traits: `MergeRequestProvider`, `UserProvider`, `RepositoryProvider`
//! 2. Implement optional traits for additional features
//! 3. Implement the `Provider` trait to tie everything together
//!
//! # Example
//!
//! ```rust,ignore
//! use stack_provider_api::{Provider, MergeRequestProvider, UserProvider, RepositoryProvider};
//!
//! pub struct MyProvider {
//!     // ... fields
//! }
//!
//! #[async_trait::async_trait]
//! impl MergeRequestProvider for MyProvider {
//!     // ... implement methods
//! }
//!
//! #[async_trait::async_trait]
//! impl UserProvider for MyProvider {
//!     // ... implement methods
//! }
//!
//! #[async_trait::async_trait]
//! impl RepositoryProvider for MyProvider {
//!     // ... implement methods
//! }
//!
//! impl Provider for MyProvider {
//!     fn name(&self) -> &'static str { "myprovider" }
//!     fn display_name(&self) -> &'static str { "My Provider" }
//!     // ... implement remaining methods
//! }
//! ```

use async_trait::async_trait;

use crate::error::ProviderResult;
use crate::types::*;

/// Core merge request operations.
///
/// This trait is required for all providers and covers the basic
/// operations needed to work with pull requests / merge requests.
#[async_trait]
pub trait MergeRequestProvider: Send + Sync {
    /// Create a new merge request.
    ///
    /// # Arguments
    ///
    /// * `repo` - The repository to create the MR in
    /// * `request` - The MR creation parameters
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Authentication fails
    /// - The source branch doesn't exist
    /// - An MR already exists for this branch
    /// - Required fields are missing
    async fn create_mr(
        &self,
        repo: &RepoId,
        request: CreateMergeRequest,
    ) -> ProviderResult<MergeRequest>;

    /// Update an existing merge request.
    ///
    /// Only the fields set in `update` will be modified.
    ///
    /// # Arguments
    ///
    /// * `repo` - The repository containing the MR
    /// * `id` - The MR identifier
    /// * `update` - The fields to update
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NotFound` if the MR doesn't exist.
    async fn update_mr(
        &self,
        repo: &RepoId,
        id: MergeRequestId,
        update: UpdateMergeRequest,
    ) -> ProviderResult<MergeRequest>;

    /// Get a merge request by ID.
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NotFound` if the MR doesn't exist.
    async fn get_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest>;

    /// List merge requests matching a filter.
    ///
    /// # Arguments
    ///
    /// * `repo` - The repository to search
    /// * `filter` - Filter criteria (state, branch, author, etc.)
    async fn list_mrs(
        &self,
        repo: &RepoId,
        filter: MergeRequestFilter,
    ) -> ProviderResult<Vec<MergeRequest>>;

    /// List merge requests for a specific source branch.
    ///
    /// This is a convenience method that filters by source branch
    /// and open state by default.
    ///
    /// # Arguments
    ///
    /// * `repo` - The repository to search
    /// * `branch` - The source branch name
    async fn list_mrs_for_branch(
        &self,
        repo: &RepoId,
        branch: &str,
    ) -> ProviderResult<Vec<MergeRequest>> {
        self.list_mrs(
            repo,
            MergeRequestFilter {
                source_branch: Some(branch.to_string()),
                state: Some(MergeRequestState::Open),
                ..Default::default()
            },
        )
        .await
    }

    /// Merge a merge request.
    ///
    /// # Arguments
    ///
    /// * `repo` - The repository containing the MR
    /// * `id` - The MR identifier
    /// * `method` - The merge method to use
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::MergeConflict` if there are conflicts.
    /// Returns `ProviderError::UnsupportedOperation` if the merge method isn't supported.
    async fn merge_mr(
        &self,
        repo: &RepoId,
        id: MergeRequestId,
        method: MergeMethod,
    ) -> ProviderResult<MergeResult>;

    /// Close a merge request without merging.
    async fn close_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest>;

    /// Reopen a closed merge request.
    ///
    /// # Errors
    ///
    /// Returns an error if the MR was already merged or cannot be reopened.
    async fn reopen_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest>;
}

/// Pipeline/CI operations.
///
/// This trait is optional and covers CI/CD pipeline operations.
/// Not all providers support all pipeline operations.
#[async_trait]
pub trait PipelineProvider: Send + Sync {
    /// Get the latest pipeline status for a ref (branch/tag/commit).
    ///
    /// Returns `None` if no pipeline exists for the ref.
    async fn get_pipeline_status(
        &self,
        repo: &RepoId,
        ref_name: &str,
    ) -> ProviderResult<Option<Pipeline>>;

    /// List pipelines for a merge request.
    async fn list_mr_pipelines(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<Vec<Pipeline>>;

    /// Trigger a new pipeline run.
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::UnsupportedOperation` if manual triggering isn't supported.
    async fn trigger_pipeline(&self, repo: &RepoId, ref_name: &str) -> ProviderResult<Pipeline>;

    /// Cancel a running pipeline.
    async fn cancel_pipeline(&self, repo: &RepoId, pipeline_id: u64) -> ProviderResult<()>;

    /// Retry a failed pipeline.
    async fn retry_pipeline(&self, repo: &RepoId, pipeline_id: u64) -> ProviderResult<Pipeline>;
}

/// Review and approval operations.
///
/// This trait covers code review functionality.
#[async_trait]
pub trait ApprovalProvider: Send + Sync {
    /// List reviews on a merge request.
    async fn list_reviews(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<Vec<Review>>;

    /// Request review from specific users.
    async fn request_review(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        reviewers: Vec<String>,
    ) -> ProviderResult<()>;

    /// Get the overall approval status of a merge request.
    ///
    /// This aggregates all reviews into a single status.
    async fn get_approval_status(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<ApprovalState>;

    /// Check if the MR has the required number of approvals.
    async fn has_required_approvals(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<bool>;
}

/// Label management operations.
#[async_trait]
pub trait LabelProvider: Send + Sync {
    /// List available labels for a repository.
    async fn list_labels(&self, repo: &RepoId) -> ProviderResult<Vec<Label>>;

    /// Add labels to a merge request.
    async fn add_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()>;

    /// Remove labels from a merge request.
    async fn remove_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()>;

    /// Set labels on a merge request (replaces existing labels).
    async fn set_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()>;
}

/// Milestone management operations.
#[async_trait]
pub trait MilestoneProvider: Send + Sync {
    /// List milestones for a repository.
    async fn list_milestones(
        &self,
        repo: &RepoId,
        state: Option<MilestoneState>,
    ) -> ProviderResult<Vec<Milestone>>;

    /// Assign a milestone to a merge request.
    async fn assign_milestone(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        milestone_id: u64,
    ) -> ProviderResult<()>;

    /// Remove milestone from a merge request.
    async fn remove_milestone(&self, repo: &RepoId, mr_id: MergeRequestId) -> ProviderResult<()>;
}

/// User and authentication operations.
///
/// This trait is required for all providers.
#[async_trait]
pub trait UserProvider: Send + Sync {
    /// Get the currently authenticated user.
    async fn current_user(&self) -> ProviderResult<User>;

    /// Validate the current authentication.
    ///
    /// Returns `true` if the credentials are valid.
    async fn validate_auth(&self) -> ProviderResult<bool>;

    /// Get a user by username.
    async fn get_user(&self, username: &str) -> ProviderResult<User>;
}

/// Repository operations.
///
/// This trait is required for all providers.
#[async_trait]
pub trait RepositoryProvider: Send + Sync {
    /// Check if the repository exists and is accessible.
    async fn check_access(&self, repo: &RepoId) -> ProviderResult<bool>;

    /// Get the default branch name for a repository.
    async fn get_default_branch(&self, repo: &RepoId) -> ProviderResult<String>;

    /// Parse a remote URL into repository identifier.
    ///
    /// Returns `None` if the URL is not recognized or doesn't match this provider.
    fn parse_remote_url(&self, url: &str) -> Option<RepoId>;
}

/// Main provider trait combining all capabilities.
///
/// This trait brings together all provider functionality and provides
/// capability discovery for optional features.
///
/// # Example
///
/// ```rust,ignore
/// use stack_provider_api::Provider;
///
/// fn use_provider(provider: &dyn Provider) {
///     // Check capabilities
///     let caps = provider.capabilities();
///     if caps.pipelines {
///         // Use pipeline features
///         if let Some(pipeline_provider) = provider.pipelines() {
///             // ...
///         }
///     }
/// }
/// ```
pub trait Provider: MergeRequestProvider + UserProvider + RepositoryProvider {
    /// Get the provider's internal name (e.g., "github", "gitlab").
    fn name(&self) -> &'static str;

    /// Get the provider's display name (e.g., "GitHub", "GitLab").
    fn display_name(&self) -> &'static str;

    /// Get the capabilities supported by this provider.
    fn capabilities(&self) -> ProviderCapabilities;

    /// Get the pipeline provider if supported.
    fn pipelines(&self) -> Option<&dyn PipelineProvider> {
        None
    }

    /// Get the approval provider if supported.
    fn approvals(&self) -> Option<&dyn ApprovalProvider> {
        None
    }

    /// Get the label provider if supported.
    fn labels(&self) -> Option<&dyn LabelProvider> {
        None
    }

    /// Get the milestone provider if supported.
    fn milestones(&self) -> Option<&dyn MilestoneProvider> {
        None
    }
}

// Allow using Box<dyn Provider> as a trait object
#[async_trait]
impl<T: Provider + ?Sized> MergeRequestProvider for Box<T> {
    async fn create_mr(
        &self,
        repo: &RepoId,
        request: CreateMergeRequest,
    ) -> ProviderResult<MergeRequest> {
        (**self).create_mr(repo, request).await
    }

    async fn update_mr(
        &self,
        repo: &RepoId,
        id: MergeRequestId,
        update: UpdateMergeRequest,
    ) -> ProviderResult<MergeRequest> {
        (**self).update_mr(repo, id, update).await
    }

    async fn get_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest> {
        (**self).get_mr(repo, id).await
    }

    async fn list_mrs(
        &self,
        repo: &RepoId,
        filter: MergeRequestFilter,
    ) -> ProviderResult<Vec<MergeRequest>> {
        (**self).list_mrs(repo, filter).await
    }

    async fn merge_mr(
        &self,
        repo: &RepoId,
        id: MergeRequestId,
        method: MergeMethod,
    ) -> ProviderResult<MergeResult> {
        (**self).merge_mr(repo, id, method).await
    }

    async fn close_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest> {
        (**self).close_mr(repo, id).await
    }

    async fn reopen_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest> {
        (**self).reopen_mr(repo, id).await
    }
}

#[async_trait]
impl<T: Provider + ?Sized> UserProvider for Box<T> {
    async fn current_user(&self) -> ProviderResult<User> {
        (**self).current_user().await
    }

    async fn validate_auth(&self) -> ProviderResult<bool> {
        (**self).validate_auth().await
    }

    async fn get_user(&self, username: &str) -> ProviderResult<User> {
        (**self).get_user(username).await
    }
}

#[async_trait]
impl<T: Provider + ?Sized> RepositoryProvider for Box<T> {
    async fn check_access(&self, repo: &RepoId) -> ProviderResult<bool> {
        (**self).check_access(repo).await
    }

    async fn get_default_branch(&self, repo: &RepoId) -> ProviderResult<String> {
        (**self).get_default_branch(repo).await
    }

    fn parse_remote_url(&self, url: &str) -> Option<RepoId> {
        (**self).parse_remote_url(url)
    }
}

impl<T: Provider + ?Sized> Provider for Box<T> {
    fn name(&self) -> &'static str {
        (**self).name()
    }

    fn display_name(&self) -> &'static str {
        (**self).display_name()
    }

    fn capabilities(&self) -> ProviderCapabilities {
        (**self).capabilities()
    }

    fn pipelines(&self) -> Option<&dyn PipelineProvider> {
        (**self).pipelines()
    }

    fn approvals(&self) -> Option<&dyn ApprovalProvider> {
        (**self).approvals()
    }

    fn labels(&self) -> Option<&dyn LabelProvider> {
        (**self).labels()
    }

    fn milestones(&self) -> Option<&dyn MilestoneProvider> {
        (**self).milestones()
    }
}
