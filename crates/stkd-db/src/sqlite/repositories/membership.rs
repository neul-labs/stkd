//! SQLite membership repository implementation.

use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{DbError, DbResult};
use crate::models::{Membership, MembershipRole, Organization, User};
use crate::repositories::MembershipRepository;

/// SQLite implementation of membership repository.
pub struct SqliteMembershipRepository {
    pool: SqlitePool,
}

impl SqliteMembershipRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MembershipRepository for SqliteMembershipRepository {
    async fn add(&self, membership: &Membership) -> DbResult<()> {
        sqlx::query(
            r#"
            INSERT INTO memberships (org_id, user_id, role, joined_at)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(membership.org_id.to_string())
        .bind(membership.user_id.to_string())
        .bind(membership.role.to_string())
        .bind(membership.joined_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                DbError::Duplicate("User is already a member of this organization".to_string())
            } else {
                DbError::Query(e.to_string())
            }
        })?;

        Ok(())
    }

    async fn get(&self, org_id: Uuid, user_id: Uuid) -> DbResult<Option<Membership>> {
        let row = sqlx::query_as::<_, MembershipRow>(
            "SELECT * FROM memberships WHERE org_id = ? AND user_id = ?",
        )
        .bind(org_id.to_string())
        .bind(user_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn update_role(&self, org_id: Uuid, user_id: Uuid, role: MembershipRole) -> DbResult<()> {
        let result =
            sqlx::query("UPDATE memberships SET role = ? WHERE org_id = ? AND user_id = ?")
                .bind(role.to_string())
                .bind(org_id.to_string())
                .bind(user_id.to_string())
                .execute(&self.pool)
                .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound("Membership not found".to_string()));
        }

        Ok(())
    }

    async fn remove(&self, org_id: Uuid, user_id: Uuid) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM memberships WHERE org_id = ? AND user_id = ?")
            .bind(org_id.to_string())
            .bind(user_id.to_string())
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound("Membership not found".to_string()));
        }

        Ok(())
    }

    async fn list_members(&self, org_id: Uuid) -> DbResult<Vec<(User, Membership)>> {
        let rows = sqlx::query_as::<_, UserMembershipRow>(
            r#"
            SELECT u.*, m.role, m.joined_at
            FROM users u
            JOIN memberships m ON u.id = m.user_id
            WHERE m.org_id = ?
            ORDER BY m.joined_at
            "#,
        )
        .bind(org_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_tuple(org_id)).collect())
    }

    async fn list_user_orgs(&self, user_id: Uuid) -> DbResult<Vec<(Organization, Membership)>> {
        let rows = sqlx::query_as::<_, OrgMembershipRow>(
            r#"
            SELECT o.*, m.role, m.joined_at
            FROM organizations o
            JOIN memberships m ON o.id = m.org_id
            WHERE m.user_id = ?
            ORDER BY o.name
            "#,
        )
        .bind(user_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_tuple(user_id)).collect())
    }

    async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> DbResult<bool> {
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM memberships WHERE org_id = ? AND user_id = ?")
                .bind(org_id.to_string())
                .bind(user_id.to_string())
                .fetch_one(&self.pool)
                .await?;

        Ok(count.0 > 0)
    }

    async fn owner_count(&self, org_id: Uuid) -> DbResult<usize> {
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM memberships WHERE org_id = ? AND role = 'owner'")
                .bind(org_id.to_string())
                .fetch_one(&self.pool)
                .await?;

        Ok(count.0 as usize)
    }
}

/// SQLite row type for memberships.
#[derive(sqlx::FromRow)]
struct MembershipRow {
    org_id: String,
    user_id: String,
    role: String,
    joined_at: chrono::DateTime<chrono::Utc>,
}

impl From<MembershipRow> for Membership {
    fn from(row: MembershipRow) -> Self {
        Self {
            org_id: Uuid::parse_str(&row.org_id).unwrap_or_default(),
            user_id: Uuid::parse_str(&row.user_id).unwrap_or_default(),
            role: row.role.parse().unwrap_or(MembershipRole::Member),
            joined_at: row.joined_at,
        }
    }
}

/// SQLite row type for user with membership.
#[derive(sqlx::FromRow)]
struct UserMembershipRow {
    id: String,
    username: String,
    email: Option<String>,
    display_name: Option<String>,
    avatar_url: Option<String>,
    provider: String,
    provider_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    role: String,
    joined_at: chrono::DateTime<chrono::Utc>,
}

impl UserMembershipRow {
    fn into_tuple(self, org_id: Uuid) -> (User, Membership) {
        let user = User {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            username: self.username,
            email: self.email,
            display_name: self.display_name,
            avatar_url: self.avatar_url,
            provider: self.provider,
            provider_id: self.provider_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
        let membership = Membership {
            org_id,
            user_id: user.id,
            role: self.role.parse().unwrap_or(MembershipRole::Member),
            joined_at: self.joined_at,
        };
        (user, membership)
    }
}

/// SQLite row type for organization with membership.
#[derive(sqlx::FromRow)]
struct OrgMembershipRow {
    id: String,
    name: String,
    slug: String,
    description: Option<String>,
    avatar_url: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    role: String,
    joined_at: chrono::DateTime<chrono::Utc>,
}

impl OrgMembershipRow {
    fn into_tuple(self, user_id: Uuid) -> (Organization, Membership) {
        let org_id = Uuid::parse_str(&self.id).unwrap_or_default();
        let org = Organization {
            id: org_id,
            name: self.name,
            slug: self.slug,
            description: self.description,
            avatar_url: self.avatar_url,
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
        let membership = Membership {
            org_id,
            user_id,
            role: self.role.parse().unwrap_or(MembershipRole::Member),
            joined_at: self.joined_at,
        };
        (org, membership)
    }
}
