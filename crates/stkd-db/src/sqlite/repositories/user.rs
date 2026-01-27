//! SQLite user repository implementation.

use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{DbError, DbResult};
use crate::models::User;
use crate::repositories::UserRepository;

/// SQLite implementation of user repository.
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create(&self, user: &User) -> DbResult<()> {
        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, display_name, avatar_url, provider, provider_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.id.to_string())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.display_name)
        .bind(&user.avatar_url)
        .bind(&user.provider)
        .bind(&user.provider_id)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> DbResult<Option<User>> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM users WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_by_provider(&self, provider: &str, provider_id: &str) -> DbResult<Option<User>> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM users WHERE provider = ? AND provider_id = ?",
        )
        .bind(provider)
        .bind(provider_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn get_by_username(&self, username: &str) -> DbResult<Option<User>> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT * FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, user: &User) -> DbResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET username = ?, email = ?, display_name = ?, avatar_url = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.display_name)
        .bind(&user.avatar_url)
        .bind(user.updated_at)
        .bind(user.id.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "User with id '{}' not found",
                user.id
            )));
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound(format!(
                "User with id '{}' not found",
                id
            )));
        }

        Ok(())
    }

    async fn find_or_create_by_oauth(
        &self,
        provider: &str,
        provider_id: &str,
        username: &str,
        email: Option<&str>,
        display_name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> DbResult<User> {
        // Try to find existing user
        if let Some(mut user) = self.get_by_provider(provider, provider_id).await? {
            // Update user info
            user.username = username.to_string();
            user.email = email.map(|s| s.to_string());
            user.display_name = display_name.map(|s| s.to_string());
            user.avatar_url = avatar_url.map(|s| s.to_string());
            user.updated_at = Utc::now();
            self.update(&user).await?;
            return Ok(user);
        }

        // Create new user
        let user = User::from_oauth(
            username.to_string(),
            email.map(|s| s.to_string()),
            display_name.map(|s| s.to_string()),
            avatar_url.map(|s| s.to_string()),
            provider.to_string(),
            provider_id.to_string(),
        );
        self.create(&user).await?;
        Ok(user)
    }
}

/// SQLite row type for users.
#[derive(sqlx::FromRow)]
struct UserRow {
    id: String,
    username: String,
    email: Option<String>,
    display_name: Option<String>,
    avatar_url: Option<String>,
    provider: String,
    provider_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            username: row.username,
            email: row.email,
            display_name: row.display_name,
            avatar_url: row.avatar_url,
            provider: row.provider,
            provider_id: row.provider_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
