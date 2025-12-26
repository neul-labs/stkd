//! Database pool trait and factory.

use async_trait::async_trait;

use crate::config::{DatabaseBackend, DatabaseConfig};
use crate::error::{DbError, DbResult};
use crate::repositories::*;

/// Database pool trait providing access to all repositories.
#[async_trait]
pub trait DatabasePool: Send + Sync {
    /// Get the organization repository.
    fn organizations(&self) -> &dyn OrganizationRepository;

    /// Get the user repository.
    fn users(&self) -> &dyn UserRepository;

    /// Get the membership repository.
    fn memberships(&self) -> &dyn MembershipRepository;

    /// Get the repository repository.
    fn repositories(&self) -> &dyn RepositoryRepository;

    /// Get the branch repository.
    fn branches(&self) -> &dyn BranchRepository;

    /// Get the merge request repository.
    fn merge_requests(&self) -> &dyn MergeRequestRepository;

    /// Get the session repository.
    fn sessions(&self) -> &dyn SessionRepository;

    /// Run database migrations.
    async fn migrate(&self) -> DbResult<()>;

    /// Check database health.
    async fn health_check(&self) -> DbResult<()>;
}

/// Create a database pool from configuration.
pub async fn create_pool(config: &DatabaseConfig) -> DbResult<Box<dyn DatabasePool>> {
    match config.backend {
        #[cfg(feature = "sqlite")]
        DatabaseBackend::Sqlite => {
            let pool = crate::sqlite::SqlitePool::connect(config).await?;
            Ok(Box::new(pool))
        }
        #[cfg(feature = "postgres")]
        DatabaseBackend::Postgres => {
            let pool = crate::postgres::PostgresPool::connect(config).await?;
            Ok(Box::new(pool))
        }
        #[allow(unreachable_patterns)]
        _ => Err(DbError::Config(format!(
            "Database backend '{}' not enabled. Enable the '{}' feature.",
            config.backend, config.backend
        ))),
    }
}
