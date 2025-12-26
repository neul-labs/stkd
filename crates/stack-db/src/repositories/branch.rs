//! Branch repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::DbResult;
use crate::models::{Branch, BranchStatus};

/// Repository for branch operations.
#[async_trait]
pub trait BranchRepository: Send + Sync {
    /// Create a new branch.
    async fn create(&self, branch: &Branch) -> DbResult<()>;

    /// Get a branch by ID.
    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<Branch>>;

    /// Get a branch by repository and name.
    async fn get_by_name(&self, repo_id: Uuid, name: &str) -> DbResult<Option<Branch>>;

    /// Update a branch.
    async fn update(&self, branch: &Branch) -> DbResult<()>;

    /// Delete a branch.
    async fn delete(&self, id: Uuid) -> DbResult<()>;

    /// List all branches in a repository.
    async fn list_by_repo(&self, repo_id: Uuid) -> DbResult<Vec<Branch>>;

    /// List children of a branch (branches that stack on it).
    async fn list_children(&self, repo_id: Uuid, parent_name: &str) -> DbResult<Vec<Branch>>;

    /// Update branch status.
    async fn update_status(&self, id: Uuid, status: BranchStatus) -> DbResult<()>;

    /// Update branch head SHA.
    async fn update_head(&self, id: Uuid, head_sha: &str) -> DbResult<()>;

    /// Associate a merge request with a branch.
    async fn set_mr(&self, id: Uuid, mr_id: Option<Uuid>) -> DbResult<()>;

    /// Find or create a branch.
    async fn find_or_create(
        &self,
        repo_id: Uuid,
        name: &str,
        parent_name: Option<&str>,
    ) -> DbResult<Branch>;
}
