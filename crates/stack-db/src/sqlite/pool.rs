//! SQLite connection pool implementation.

use async_trait::async_trait;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool as SqlxPool;
use std::str::FromStr;
use std::sync::Arc;

use crate::config::DatabaseConfig;
use crate::error::{DbError, DbResult};
use crate::pool::DatabasePool;
use crate::repositories::*;

use super::repositories::*;

/// SQLite database pool.
pub struct SqlitePool {
    pool: SqlxPool,
    organizations: Arc<SqliteOrganizationRepository>,
    users: Arc<SqliteUserRepository>,
    memberships: Arc<SqliteMembershipRepository>,
    repositories: Arc<SqliteRepositoryRepository>,
    branches: Arc<SqliteBranchRepository>,
    merge_requests: Arc<SqliteMergeRequestRepository>,
    sessions: Arc<SqliteSessionRepository>,
}

impl SqlitePool {
    /// Connect to the SQLite database.
    pub async fn connect(config: &DatabaseConfig) -> DbResult<Self> {
        let options = SqliteConnectOptions::from_str(&config.url)
            .map_err(|e| DbError::Connection(e.to_string()))?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(std::time::Duration::from_secs(30));

        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect_with(options)
            .await
            .map_err(|e| DbError::Connection(e.to_string()))?;

        Ok(Self {
            pool: pool.clone(),
            organizations: Arc::new(SqliteOrganizationRepository::new(pool.clone())),
            users: Arc::new(SqliteUserRepository::new(pool.clone())),
            memberships: Arc::new(SqliteMembershipRepository::new(pool.clone())),
            repositories: Arc::new(SqliteRepositoryRepository::new(pool.clone())),
            branches: Arc::new(SqliteBranchRepository::new(pool.clone())),
            merge_requests: Arc::new(SqliteMergeRequestRepository::new(pool.clone())),
            sessions: Arc::new(SqliteSessionRepository::new(pool)),
        })
    }

    /// Run migrations.
    async fn run_migrations(&self) -> DbResult<()> {
        sqlx::query(include_str!("../../migrations/sqlite/001_initial.sql"))
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::Migration(e.to_string()))?;

        tracing::info!("SQLite migrations completed successfully");
        Ok(())
    }
}

#[async_trait]
impl DatabasePool for SqlitePool {
    fn organizations(&self) -> &dyn OrganizationRepository {
        self.organizations.as_ref()
    }

    fn users(&self) -> &dyn UserRepository {
        self.users.as_ref()
    }

    fn memberships(&self) -> &dyn MembershipRepository {
        self.memberships.as_ref()
    }

    fn repositories(&self) -> &dyn RepositoryRepository {
        self.repositories.as_ref()
    }

    fn branches(&self) -> &dyn BranchRepository {
        self.branches.as_ref()
    }

    fn merge_requests(&self) -> &dyn MergeRequestRepository {
        self.merge_requests.as_ref()
    }

    fn sessions(&self) -> &dyn SessionRepository {
        self.sessions.as_ref()
    }

    async fn migrate(&self) -> DbResult<()> {
        self.run_migrations().await
    }

    async fn health_check(&self) -> DbResult<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;
        Ok(())
    }
}
