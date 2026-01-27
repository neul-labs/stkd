//! User model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A user account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier
    pub id: Uuid,
    /// Username (from OAuth provider)
    pub username: String,
    /// Email address
    pub email: Option<String>,
    /// Display name
    pub display_name: Option<String>,
    /// Avatar URL
    pub avatar_url: Option<String>,
    /// OAuth provider (github, gitlab)
    pub provider: String,
    /// Provider-specific user ID
    pub provider_id: String,
    /// When the user was created
    pub created_at: DateTime<Utc>,
    /// When the user was last updated
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Create a new user from OAuth data.
    pub fn from_oauth(
        username: String,
        email: Option<String>,
        display_name: Option<String>,
        avatar_url: Option<String>,
        provider: String,
        provider_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            display_name,
            avatar_url,
            provider,
            provider_id,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the display name or fall back to username.
    pub fn name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.username)
    }
}
