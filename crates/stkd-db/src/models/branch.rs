//! Branch model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Branch status in the stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BranchStatus {
    /// Branch is active and has an open MR
    Active,
    /// Branch has been merged
    Merged,
    /// Branch has been closed without merging
    Closed,
    /// Branch has no MR yet
    Local,
}

impl std::fmt::Display for BranchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BranchStatus::Active => write!(f, "active"),
            BranchStatus::Merged => write!(f, "merged"),
            BranchStatus::Closed => write!(f, "closed"),
            BranchStatus::Local => write!(f, "local"),
        }
    }
}

impl std::str::FromStr for BranchStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(BranchStatus::Active),
            "merged" => Ok(BranchStatus::Merged),
            "closed" => Ok(BranchStatus::Closed),
            "local" => Ok(BranchStatus::Local),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

/// A branch in a repository stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    /// Unique identifier
    pub id: Uuid,
    /// Repository this branch belongs to
    pub repo_id: Uuid,
    /// Branch name
    pub name: String,
    /// Parent branch name (the branch this stacks on)
    pub parent_name: Option<String>,
    /// Associated merge request ID (if any)
    pub mr_id: Option<Uuid>,
    /// Current status
    pub status: BranchStatus,
    /// Latest commit SHA
    pub head_sha: Option<String>,
    /// When the branch was created
    pub created_at: DateTime<Utc>,
    /// When the branch was last updated
    pub updated_at: DateTime<Utc>,
}

impl Branch {
    /// Create a new branch.
    pub fn new(repo_id: Uuid, name: String, parent_name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            repo_id,
            name,
            parent_name,
            mr_id: None,
            status: BranchStatus::Local,
            head_sha: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if this branch is the root of a stack.
    pub fn is_root(&self) -> bool {
        self.parent_name.is_none()
    }
}
