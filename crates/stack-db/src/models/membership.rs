//! Organization membership model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Role in an organization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MembershipRole {
    /// Organization owner - full permissions
    Owner,
    /// Administrator - can manage members and settings
    Admin,
    /// Regular member - can view and use
    Member,
}

impl std::fmt::Display for MembershipRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MembershipRole::Owner => write!(f, "owner"),
            MembershipRole::Admin => write!(f, "admin"),
            MembershipRole::Member => write!(f, "member"),
        }
    }
}

impl std::str::FromStr for MembershipRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "owner" => Ok(MembershipRole::Owner),
            "admin" => Ok(MembershipRole::Admin),
            "member" => Ok(MembershipRole::Member),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

/// Organization membership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    /// Organization ID
    pub org_id: Uuid,
    /// User ID
    pub user_id: Uuid,
    /// Role in the organization
    pub role: MembershipRole,
    /// When the membership was created
    pub joined_at: DateTime<Utc>,
}

impl Membership {
    /// Create a new membership.
    pub fn new(org_id: Uuid, user_id: Uuid, role: MembershipRole) -> Self {
        Self {
            org_id,
            user_id,
            role,
            joined_at: Utc::now(),
        }
    }

    /// Check if this membership has admin privileges.
    pub fn is_admin(&self) -> bool {
        matches!(self.role, MembershipRole::Owner | MembershipRole::Admin)
    }

    /// Check if this membership is an owner.
    pub fn is_owner(&self) -> bool {
        matches!(self.role, MembershipRole::Owner)
    }
}
