//! Repository repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::DbResult;
use crate::models::Repository;

/// Repository for repository operations.
#[async_trait]
pub trait RepositoryRepository: Send + Sync {
    /// Create a new repository.
    async fn create(&self, repo: &Repository) -> DbResult<()>;

    /// Get a repository by ID.
    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<Repository>>;

    /// Get a repository by provider details.
    async fn get_by_provider(
        &self,
        provider: &str,
        owner: &str,
        name: &str,
    ) -> DbResult<Option<Repository>>;

    /// Update a repository.
    async fn update(&self, repo: &Repository) -> DbResult<()>;

    /// Delete a repository.
    async fn delete(&self, id: Uuid) -> DbResult<()>;

    /// List all repositories in an organization.
    async fn list_by_org(&self, org_id: Uuid) -> DbResult<Vec<Repository>>;

    /// Mark a repository as synced.
    async fn mark_synced(&self, id: Uuid) -> DbResult<()>;

    /// Set repository active status.
    async fn set_active(&self, id: Uuid, is_active: bool) -> DbResult<()>;
}
