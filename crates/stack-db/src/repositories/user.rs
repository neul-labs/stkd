//! User repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::DbResult;
use crate::models::User;

/// Repository for user operations.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user.
    async fn create(&self, user: &User) -> DbResult<()>;

    /// Get a user by ID.
    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<User>>;

    /// Get a user by provider and provider ID.
    async fn get_by_provider(&self, provider: &str, provider_id: &str) -> DbResult<Option<User>>;

    /// Get a user by username.
    async fn get_by_username(&self, username: &str) -> DbResult<Option<User>>;

    /// Update a user.
    async fn update(&self, user: &User) -> DbResult<()>;

    /// Delete a user.
    async fn delete(&self, id: Uuid) -> DbResult<()>;

    /// Find or create a user from OAuth data.
    async fn find_or_create_by_oauth(
        &self,
        provider: &str,
        provider_id: &str,
        username: &str,
        email: Option<&str>,
        display_name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> DbResult<User>;
}
