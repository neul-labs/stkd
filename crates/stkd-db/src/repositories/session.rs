//! Session repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::DbResult;
use crate::models::Session;

/// Repository for session operations.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Create a new session.
    async fn create(&self, session: &Session) -> DbResult<()>;

    /// Get a session by ID.
    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<Session>>;

    /// Get a session by token hash.
    async fn get_by_token(&self, token_hash: &str) -> DbResult<Option<Session>>;

    /// Delete a session.
    async fn delete(&self, id: Uuid) -> DbResult<()>;

    /// Delete all sessions for a user.
    async fn delete_all_for_user(&self, user_id: Uuid) -> DbResult<u64>;

    /// Delete expired sessions.
    async fn delete_expired(&self) -> DbResult<u64>;

    /// List all sessions for a user.
    async fn list_by_user(&self, user_id: Uuid) -> DbResult<Vec<Session>>;

    /// Validate a session token and return the session if valid.
    async fn validate(&self, token_hash: &str) -> DbResult<Option<Session>>;
}
