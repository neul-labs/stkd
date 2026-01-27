//! Organization repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::DbResult;
use crate::models::Organization;

/// Repository for organization operations.
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    /// Create a new organization.
    async fn create(&self, org: &Organization) -> DbResult<()>;

    /// Get an organization by ID.
    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<Organization>>;

    /// Get an organization by slug.
    async fn get_by_slug(&self, slug: &str) -> DbResult<Option<Organization>>;

    /// Update an organization.
    async fn update(&self, org: &Organization) -> DbResult<()>;

    /// Delete an organization.
    async fn delete(&self, id: Uuid) -> DbResult<()>;

    /// List all organizations.
    async fn list_all(&self) -> DbResult<Vec<Organization>>;

    /// Check if a slug is available.
    async fn slug_exists(&self, slug: &str) -> DbResult<bool>;
}
