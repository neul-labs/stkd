//! SQLite merge request repository implementation.

use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{DbError, DbResult};
use crate::models::{MergeRequest, MergeRequestState};
use crate::repositories::MergeRequestRepository;

/// SQLite implementation of merge request repository.
pub struct SqliteMergeRequestRepository {
    pool: SqlitePool,
}

impl SqliteMergeRequestRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MergeRequestRepository for SqliteMergeRequestRepository {
    async fn create(&self, mr: &MergeRequest) -> DbResult<()> {
        sqlx::query(
            r#"
            INSERT INTO merge_requests (id, repo_id, branch_id, number, title, body, state, url, source_branch, target_branch, provider_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(mr.id.to_string())
        .bind(mr.repo_id.to_string())
        .bind(mr.branch_id.to_string())
        .bind(mr.number as i64)
        .bind(&mr.title)
        .bind(&mr.body)
        .bind(mr.state.to_string())
        .bind(&mr.url)
        .bind(&mr.source_branch)
        .bind(&mr.target_branch)
        .bind(&mr.provider_id)
        .bind(mr.created_at)
        .bind(mr.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<MergeRequest>> {
        let row = sqlx::query_as::<_, MergeRequestRow>("SELECT * FROM merge_requests WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_by_number(&self, repo_id: Uuid, number: u64) -> DbResult<Option<MergeRequest>> {
        let row = sqlx::query_as::<_, MergeRequestRow>(
            "SELECT * FROM merge_requests WHERE repo_id = ? AND number = ?",
        )
        .bind(repo_id.to_string())
        .bind(number as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_by_branch(&self, branch_id: Uuid) -> DbResult<Option<MergeRequest>> {
        let row = sqlx::query_as::<_, MergeRequestRow>(
            "SELECT * FROM merge_requests WHERE branch_id = ?",
        )
        .bind(branch_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, mr: &MergeRequest) -> DbResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE merge_requests
            SET title = ?, body = ?, state = ?, source_branch = ?, target_branch = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&mr.title)
        .bind(&mr.body)
        .bind(mr.state.to_string())
        .bind(&mr.source_branch)
        .bind(&mr.target_branch)
        .bind(mr.updated_at)
        .bind(mr.id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Merge request with id '{}' not found",
                mr.id
            )));
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM merge_requests WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Merge request with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn list_by_repo(&self, repo_id: Uuid) -> DbResult<Vec<MergeRequest>> {
        let rows = sqlx::query_as::<_, MergeRequestRow>(
            "SELECT * FROM merge_requests WHERE repo_id = ? ORDER BY number DESC",
        )
        .bind(repo_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn list_open(&self, repo_id: Uuid) -> DbResult<Vec<MergeRequest>> {
        let rows = sqlx::query_as::<_, MergeRequestRow>(
            "SELECT * FROM merge_requests WHERE repo_id = ? AND (state = 'open' OR state = 'draft') ORDER BY number DESC",
        )
        .bind(repo_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update_state(&self, id: Uuid, state: MergeRequestState) -> DbResult<()> {
        let result =
            sqlx::query("UPDATE merge_requests SET state = ?, updated_at = ? WHERE id = ?")
                .bind(state.to_string())
                .bind(Utc::now())
                .bind(id.to_string())
                .execute(&self.pool)
                .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Merge request with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

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
    ) -> DbResult<MergeRequest> {
        if let Some(mut mr) = self.get_by_number(repo_id, number).await? {
            // Update existing MR
            mr.title = title.to_string();
            mr.url = url.to_string();
            mr.source_branch = source_branch.to_string();
            mr.target_branch = target_branch.to_string();
            mr.updated_at = Utc::now();
            self.update(&mr).await?;
            return Ok(mr);
        }

        let mr = MergeRequest::new(
            repo_id,
            branch_id,
            number,
            title.to_string(),
            None,
            url.to_string(),
            source_branch.to_string(),
            target_branch.to_string(),
            provider_id.to_string(),
        );
        self.create(&mr).await?;
        Ok(mr)
    }
}

/// SQLite row type for merge requests.
#[derive(sqlx::FromRow)]
struct MergeRequestRow {
    id: String,
    repo_id: String,
    branch_id: String,
    number: i64,
    title: String,
    body: Option<String>,
    state: String,
    url: String,
    source_branch: String,
    target_branch: String,
    provider_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<MergeRequestRow> for MergeRequest {
    fn from(row: MergeRequestRow) -> Self {
        Self {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            repo_id: Uuid::parse_str(&row.repo_id).unwrap_or_default(),
            branch_id: Uuid::parse_str(&row.branch_id).unwrap_or_default(),
            number: row.number as u64,
            title: row.title,
            body: row.body,
            state: row.state.parse().unwrap_or(MergeRequestState::Open),
            url: row.url,
            source_branch: row.source_branch,
            target_branch: row.target_branch,
            provider_id: row.provider_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
