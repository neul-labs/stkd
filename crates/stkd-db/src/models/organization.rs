//! Organization model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An organization (team) that owns repositories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    /// Unique identifier
    pub id: Uuid,
    /// Display name
    pub name: String,
    /// URL-safe slug (unique)
    pub slug: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional avatar URL
    pub avatar_url: Option<String>,
    /// When the organization was created
    pub created_at: DateTime<Utc>,
    /// When the organization was last updated
    pub updated_at: DateTime<Utc>,
}

impl Organization {
    /// Create a new organization.
    pub fn new(name: String, slug: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            description: None,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        }
    }
}
