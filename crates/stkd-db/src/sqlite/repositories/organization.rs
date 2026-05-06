//! SQLite organization repository implementation.

use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{DbError, DbResult};
use crate::models::Organization;
use crate::repositories::OrganizationRepository;

/// SQLite implementation of organization repository.
pub struct SqliteOrganizationRepository {
    pool: SqlitePool,
}

impl SqliteOrganizationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationRepository for SqliteOrganizationRepository {
    async fn create(&self, org: &Organization) -> DbResult<()> {
        sqlx::query(
            r#"
            INSERT INTO organizations (id, name, slug, description, avatar_url, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(org.id.to_string())
        .bind(&org.name)
        .bind(&org.slug)
        .bind(&org.description)
        .bind(&org.avatar_url)
        .bind(org.created_at)
        .bind(org.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                DbError::Duplicate(format!("Organization with slug '{}' already exists", org.slug))
            } else {
                DbError::Query(e.to_string())
            }
        })?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<Organization>> {
        let row = sqlx::query_as::<_, OrganizationRow>("SELECT * FROM organizations WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_by_slug(&self, slug: &str) -> DbResult<Option<Organization>> {
        let row =
            sqlx::query_as::<_, OrganizationRow>("SELECT * FROM organizations WHERE slug = ?")
                .bind(slug)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, org: &Organization) -> DbResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE organizations
            SET name = ?, slug = ?, description = ?, avatar_url = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&org.name)
        .bind(&org.slug)
        .bind(&org.description)
        .bind(&org.avatar_url)
        .bind(org.updated_at)
        .bind(org.id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Organization with id '{}' not found",
                org.id
            )));
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM organizations WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "Organization with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn list_all(&self) -> DbResult<Vec<Organization>> {
        let rows =
            sqlx::query_as::<_, OrganizationRow>("SELECT * FROM organizations ORDER BY name")
                .fetch_all(&self.pool)
                .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn slug_exists(&self, slug: &str) -> DbResult<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM organizations WHERE slug = ?")
            .bind(slug)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0 > 0)
    }
}

/// SQLite row type for organizations.
#[derive(sqlx::FromRow)]
struct OrganizationRow {
    id: String,
    name: String,
    slug: String,
    description: Option<String>,
    avatar_url: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<OrganizationRow> for Organization {
    fn from(row: OrganizationRow) -> Self {
        Self {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            name: row.name,
            slug: row.slug,
            description: row.description,
            avatar_url: row.avatar_url,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
