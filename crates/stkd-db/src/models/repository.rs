//! Repository model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A connected Git repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// Unique identifier
    pub id: Uuid,
    /// Organization that owns this repository
    pub org_id: Uuid,
    /// Provider type (github, gitlab)
    pub provider: String,
    /// Repository owner on the provider
    pub owner: String,
    /// Repository name on the provider
    pub name: String,
    /// Default branch name
    pub default_branch: String,
    /// Provider-specific repository ID
    pub provider_id: String,
    /// Whether the repository is active
    pub is_active: bool,
    /// When the repository was connected
    pub created_at: DateTime<Utc>,
    /// When the repository was last synced
    pub synced_at: Option<DateTime<Utc>>,
}

impl Repository {
    /// Create a new repository.
    pub fn new(
        org_id: Uuid,
        provider: String,
        owner: String,
        name: String,
        default_branch: String,
        provider_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            org_id,
            provider,
            owner,
            name,
            default_branch,
            provider_id,
            is_active: true,
            created_at: Utc::now(),
            synced_at: None,
        }
    }

    /// Get the full name (owner/name).
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}
