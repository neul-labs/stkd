//! SQLite repository repository implementation.

use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{DbError, DbResult};
use crate::models::Repository;
use crate::repositories::RepositoryRepository;

/// SQLite implementation of repository repository.
pub struct SqliteRepositoryRepository {
    pool: SqlitePool,
}

impl SqliteRepositoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RepositoryRepository for SqliteRepositoryRepository {
    async fn create(&self, repo: &Repository) -> DbResult<()> {
        sqlx::query(
            r#"
            INSERT INTO repositories (id, org_id, provider, owner, name, default_branch, provider_id, is_active, created_at, synced_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(repo.id.to_string())
        .bind(repo.org_id.to_string())
        .bind(&repo.provider)
        .bind(&repo.owner)
        .bind(&repo.name)
        .bind(&repo.default_branch)
        .bind(&repo.provider_id)
        .bind(repo.is_active)
        .bind(repo.created_at)
        .bind(repo.synced_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<Repository>> {
        let row = sqlx::query_as::<_, RepositoryRow>(
            "SELECT * FROM repositories WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_by_provider(
        &self,
        provider: &str,
        owner: &str,
        name: &str,
    ) -> DbResult<Option<Repository>> {
        let row = sqlx::query_as::<_, RepositoryRow>(
            "SELECT * FROM repositories WHERE provider = ? AND owner = ? AND name = ?",
        )
        .bind(provider)
        .bind(owner)
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, repo: &Repository) -> DbResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE repositories
            SET provider = ?, owner = ?, name = ?, default_branch = ?, provider_id = ?, is_active = ?, synced_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&repo.provider)
        .bind(&repo.owner)
        .bind(&repo.name)
        .bind(&repo.default_branch)
        .bind(&repo.provider_id)
        .bind(repo.is_active)
        .bind(repo.synced_at)
        .bind(repo.id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Repository with id '{}' not found",
                repo.id
            )));
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM repositories WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Repository with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn list_by_org(&self, org_id: Uuid) -> DbResult<Vec<Repository>> {
        let rows = sqlx::query_as::<_, RepositoryRow>(
            "SELECT * FROM repositories WHERE org_id = ? ORDER BY name",
        )
        .bind(org_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn mark_synced(&self, id: Uuid) -> DbResult<()> {
        let result = sqlx::query(
            "UPDATE repositories SET synced_at = ? WHERE id = ?",
        )
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Repository with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn set_active(&self, id: Uuid, is_active: bool) -> DbResult<()> {
        let result = sqlx::query(
            "UPDATE repositories SET is_active = ? WHERE id = ?",
        )
        .bind(is_active)
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Repository with id '{}' not found",
                id
            )));
        }

        Ok(())
    }
}

/// SQLite row type for repositories.
#[derive(sqlx::FromRow)]
struct RepositoryRow {
    id: String,
    org_id: String,
    provider: String,
    owner: String,
    name: String,
    default_branch: String,
    provider_id: String,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    synced_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<RepositoryRow> for Repository {
    fn from(row: RepositoryRow) -> Self {
        Self {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            org_id: Uuid::parse_str(&row.org_id).unwrap_or_default(),
            provider: row.provider,
            owner: row.owner,
            name: row.name,
            default_branch: row.default_branch,
            provider_id: row.provider_id,
            is_active: row.is_active,
            created_at: row.created_at,
            synced_at: row.synced_at,
        }
    }
}
