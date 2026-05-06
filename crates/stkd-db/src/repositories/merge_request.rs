//! Merge request repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::DbResult;
use crate::models::{MergeRequest, MergeRequestState};

/// Repository for merge request operations.
#[async_trait]
pub trait MergeRequestRepository: Send + Sync {
    /// Create a new merge request.
    async fn create(&self, mr: &MergeRequest) -> DbResult<()>;

    /// Get a merge request by ID.
    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<MergeRequest>>;

    /// Get a merge request by repository and number.
    async fn get_by_number(&self, repo_id: Uuid, number: u64) -> DbResult<Option<MergeRequest>>;

    /// Get a merge request by branch ID.
    async fn get_by_branch(&self, branch_id: Uuid) -> DbResult<Option<MergeRequest>>;

    /// Update a merge request.
    async fn update(&self, mr: &MergeRequest) -> DbResult<()>;

    /// Delete a merge request.
    async fn delete(&self, id: Uuid) -> DbResult<()>;

    /// List all merge requests in a repository.
    async fn list_by_repo(&self, repo_id: Uuid) -> DbResult<Vec<MergeRequest>>;

    /// List open merge requests in a repository.
    async fn list_open(&self, repo_id: Uuid) -> DbResult<Vec<MergeRequest>>;

    /// Update merge request state.
    async fn update_state(&self, id: Uuid, state: MergeRequestState) -> DbResult<()>;

    /// Find or create a merge request.
    #[allow(clippy::too_many_arguments)]
    async fn find_or_create_by_number(
        &self,
        repo_id: Uuid,
        branch_id: Uuid,
        number: u64,
        title: &str,
        url: &str,
        source_branch: &str,
        target_branch: &str,
        provider_id: &str,
    ) -> DbResult<MergeRequest>;
}
