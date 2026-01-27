//! Merge request model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Merge request state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MergeRequestState {
    /// Open and ready for review
    Open,
    /// Draft/WIP state
    Draft,
    /// Has been merged
    Merged,
    /// Has been closed without merging
    Closed,
}

impl std::fmt::Display for MergeRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MergeRequestState::Open => write!(f, "open"),
            MergeRequestState::Draft => write!(f, "draft"),
            MergeRequestState::Merged => write!(f, "merged"),
            MergeRequestState::Closed => write!(f, "closed"),
        }
    }
}

impl std::str::FromStr for MergeRequestState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(MergeRequestState::Open),
            "draft" => Ok(MergeRequestState::Draft),
            "merged" => Ok(MergeRequestState::Merged),
            "closed" => Ok(MergeRequestState::Closed),
            _ => Err(format!("Unknown state: {}", s)),
        }
    }
}

/// A merge/pull request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    /// Unique identifier
    pub id: Uuid,
    /// Repository this MR belongs to
    pub repo_id: Uuid,
    /// Associated branch ID
    pub branch_id: Uuid,
    /// MR number on the provider
    pub number: u64,
    /// MR title
    pub title: String,
    /// MR description/body
    pub body: Option<String>,
    /// Current state
    pub state: MergeRequestState,
    /// URL to the MR on the provider
    pub url: String,
    /// Source branch name
    pub source_branch: String,
    /// Target branch name
    pub target_branch: String,
    /// Provider-specific MR ID
    pub provider_id: String,
    /// When the MR was created
    pub created_at: DateTime<Utc>,
    /// When the MR was last updated
    pub updated_at: DateTime<Utc>,
}

impl MergeRequest {
    /// Create a new merge request.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        repo_id: Uuid,
        branch_id: Uuid,
        number: u64,
        title: String,
        body: Option<String>,
        url: String,
        source_branch: String,
        target_branch: String,
        provider_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            repo_id,
            branch_id,
            number,
            title,
            body,
            state: MergeRequestState::Open,
            url,
            source_branch,
            target_branch,
            provider_id,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if this MR is still open.
    pub fn is_open(&self) -> bool {
        matches!(self.state, MergeRequestState::Open | MergeRequestState::Draft)
    }

    /// Check if this MR has been merged.
    pub fn is_merged(&self) -> bool {
        matches!(self.state, MergeRequestState::Merged)
    }
}
