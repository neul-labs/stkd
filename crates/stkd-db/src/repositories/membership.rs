//! Membership repository trait.

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::DbResult;
use crate::models::{Membership, MembershipRole, Organization, User};

/// Repository for membership operations.
#[async_trait]
pub trait MembershipRepository: Send + Sync {
    /// Add a user to an organization.
    async fn add(&self, membership: &Membership) -> DbResult<()>;

    /// Get a membership.
    async fn get(&self, org_id: Uuid, user_id: Uuid) -> DbResult<Option<Membership>>;

    /// Update a membership role.
    async fn update_role(&self, org_id: Uuid, user_id: Uuid, role: MembershipRole) -> DbResult<()>;

    /// Remove a user from an organization.
    async fn remove(&self, org_id: Uuid, user_id: Uuid) -> DbResult<()>;

    /// List all members of an organization.
    async fn list_members(&self, org_id: Uuid) -> DbResult<Vec<(User, Membership)>>;

    /// List all organizations a user belongs to.
    async fn list_user_orgs(&self, user_id: Uuid) -> DbResult<Vec<(Organization, Membership)>>;

    /// Check if a user is a member of an organization.
    async fn is_member(&self, org_id: Uuid, user_id: Uuid) -> DbResult<bool>;

    /// Get the owner count for an organization.
    async fn owner_count(&self, org_id: Uuid) -> DbResult<usize>;
}
