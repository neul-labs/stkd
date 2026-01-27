//! SQLite branch repository implementation.

use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{DbError, DbResult};
use crate::models::{Branch, BranchStatus};
use crate::repositories::BranchRepository;

/// SQLite implementation of branch repository.
pub struct SqliteBranchRepository {
    pool: SqlitePool,
}

impl SqliteBranchRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BranchRepository for SqliteBranchRepository {
    async fn create(&self, branch: &Branch) -> DbResult<()> {
        sqlx::query(
            r#"
            INSERT INTO branches (id, repo_id, name, parent_name, mr_id, status, head_sha, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(branch.id.to_string())
        .bind(branch.repo_id.to_string())
        .bind(&branch.name)
        .bind(&branch.parent_name)
        .bind(branch.mr_id.map(|id| id.to_string()))
        .bind(branch.status.to_string())
        .bind(&branch.head_sha)
        .bind(branch.created_at)
        .bind(branch.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<Branch>> {
        let row = sqlx::query_as::<_, BranchRow>(
            "SELECT * FROM branches WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_by_name(&self, repo_id: Uuid, name: &str) -> DbResult<Option<Branch>> {
        let row = sqlx::query_as::<_, BranchRow>(
            "SELECT * FROM branches WHERE repo_id = ? AND name = ?",
        )
        .bind(repo_id.to_string())
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, branch: &Branch) -> DbResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE branches
            SET name = ?, parent_name = ?, mr_id = ?, status = ?, head_sha = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&branch.name)
        .bind(&branch.parent_name)
        .bind(branch.mr_id.map(|id| id.to_string()))
        .bind(branch.status.to_string())
        .bind(&branch.head_sha)
        .bind(branch.updated_at)
        .bind(branch.id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Branch with id '{}' not found",
                branch.id
            )));
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM branches WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Branch with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn list_by_repo(&self, repo_id: Uuid) -> DbResult<Vec<Branch>> {
        let rows = sqlx::query_as::<_, BranchRow>(
            "SELECT * FROM branches WHERE repo_id = ? ORDER BY name",
        )
        .bind(repo_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn list_children(&self, repo_id: Uuid, parent_name: &str) -> DbResult<Vec<Branch>> {
        let rows = sqlx::query_as::<_, BranchRow>(
            "SELECT * FROM branches WHERE repo_id = ? AND parent_name = ? ORDER BY name",
        )
        .bind(repo_id.to_string())
        .bind(parent_name)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update_status(&self, id: Uuid, status: BranchStatus) -> DbResult<()> {
        let result = sqlx::query(
            "UPDATE branches SET status = ?, updated_at = ? WHERE id = ?",
        )
        .bind(status.to_string())
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Branch with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn update_head(&self, id: Uuid, head_sha: &str) -> DbResult<()> {
        let result = sqlx::query(
            "UPDATE branches SET head_sha = ?, updated_at = ? WHERE id = ?",
        )
        .bind(head_sha)
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Branch with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn set_mr(&self, id: Uuid, mr_id: Option<Uuid>) -> DbResult<()> {
        let result = sqlx::query(
            "UPDATE branches SET mr_id = ?, updated_at = ? WHERE id = ?",
        )
        .bind(mr_id.map(|id| id.to_string()))
        .bind(Utc::now())
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Branch with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn find_or_create(
        &self,
        repo_id: Uuid,
        name: &str,
        parent_name: Option<&str>,
    ) -> DbResult<Branch> {
        if let Some(branch) = self.get_by_name(repo_id, name).await? {
            return Ok(branch);
        }

        let branch = Branch::new(repo_id, name.to_string(), parent_name.map(|s| s.to_string()));
        self.create(&branch).await?;
        Ok(branch)
    }
}

/// SQLite row type for branches.
#[derive(sqlx::FromRow)]
struct BranchRow {
    id: String,
    repo_id: String,
    name: String,
    parent_name: Option<String>,
    mr_id: Option<String>,
    status: String,
    head_sha: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<BranchRow> for Branch {
    fn from(row: BranchRow) -> Self {
        Self {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            repo_id: Uuid::parse_str(&row.repo_id).unwrap_or_default(),
            name: row.name,
            parent_name: row.parent_name,
            mr_id: row.mr_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: row.status.parse().unwrap_or(BranchStatus::Local),
            head_sha: row.head_sha,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
