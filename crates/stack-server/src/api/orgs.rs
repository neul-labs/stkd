//! Organization API routes.

use axum::{
    extract::{Path, State},
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

/// Create organization request.
#[derive(Debug, Deserialize)]
pub struct CreateOrgRequest {
    name: String,
    slug: String,
    description: Option<String>,
}

/// Update organization request.
#[derive(Debug, Deserialize)]
pub struct UpdateOrgRequest {
    name: Option<String>,
    description: Option<String>,
}

/// Organization response.
#[derive(Debug, Serialize)]
pub struct OrgResponse {
    id: String,
    name: String,
    slug: String,
    description: Option<String>,
    avatar_url: Option<String>,
    role: String,
}

/// Member response.
#[derive(Debug, Serialize)]
pub struct MemberResponse {
    id: String,
    username: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
    role: String,
}

/// Invite member request.
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields are read by serde deserialization
pub struct InviteMemberRequest {
    username: String,
    role: String,
}

/// List organizations for the current user.
async fn list_orgs(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> ApiResult<Json<Vec<OrgResponse>>> {
    let orgs = state
        .db()
        .memberships()
        .list_user_orgs(auth_user.user_id)
        .await?;

    let response: Vec<OrgResponse> = orgs
        .into_iter()
        .map(|(org, membership)| OrgResponse {
            id: org.id.to_string(),
            name: org.name,
            slug: org.slug,
            description: org.description,
            avatar_url: org.avatar_url,
            role: membership.role.to_string(),
        })
        .collect();

    Ok(Json(response))
}

/// Create a new organization.
async fn create_org(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateOrgRequest>,
) -> ApiResult<Json<OrgResponse>> {
    // Check if slug is available
    if state.db().organizations().slug_exists(&req.slug).await? {
        return Err(ApiError::Conflict(format!(
            "Organization with slug '{}' already exists",
            req.slug
        )));
    }

    // Create organization
    let mut org = stack_db::Organization::new(req.name.clone(), req.slug);
    org.description = req.description;
    state.db().organizations().create(&org).await?;

    // Add creator as owner
    let membership = stack_db::Membership::new(
        org.id,
        auth_user.user_id,
        stack_db::MembershipRole::Owner,
    );
    state.db().memberships().add(&membership).await?;

    Ok(Json(OrgResponse {
        id: org.id.to_string(),
        name: org.name,
        slug: org.slug,
        description: org.description,
        avatar_url: org.avatar_url,
        role: "owner".to_string(),
    }))
}

/// Get organization by slug.
async fn get_org(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<OrgResponse>> {
    let org = state
        .db()
        .organizations()
        .get_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    // Check membership
    let membership = state
        .db()
        .memberships()
        .get(org.id, auth_user.user_id)
        .await?
        .ok_or_else(|| ApiError::Forbidden("Not a member of this organization".to_string()))?;

    Ok(Json(OrgResponse {
        id: org.id.to_string(),
        name: org.name,
        slug: org.slug,
        description: org.description,
        avatar_url: org.avatar_url,
        role: membership.role.to_string(),
    }))
}

/// Update organization.
async fn update_org(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
    Json(req): Json<UpdateOrgRequest>,
) -> ApiResult<Json<OrgResponse>> {
    let mut org = state
        .db()
        .organizations()
        .get_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    // Check admin permission
    let membership = state
        .db()
        .memberships()
        .get(org.id, auth_user.user_id)
        .await?
        .ok_or_else(|| ApiError::Forbidden("Not a member of this organization".to_string()))?;

    if !membership.is_admin() {
        return Err(ApiError::Forbidden("Admin permission required".to_string()));
    }

    // Update fields
    if let Some(name) = req.name {
        org.name = name;
    }
    if let Some(description) = req.description {
        org.description = Some(description);
    }
    org.updated_at = chrono::Utc::now();

    state.db().organizations().update(&org).await?;

    Ok(Json(OrgResponse {
        id: org.id.to_string(),
        name: org.name,
        slug: org.slug,
        description: org.description,
        avatar_url: org.avatar_url,
        role: membership.role.to_string(),
    }))
}

/// Delete organization.
async fn delete_org(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let org = state
        .db()
        .organizations()
        .get_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    // Check owner permission
    let membership = state
        .db()
        .memberships()
        .get(org.id, auth_user.user_id)
        .await?
        .ok_or_else(|| ApiError::Forbidden("Not a member of this organization".to_string()))?;

    if !membership.is_owner() {
        return Err(ApiError::Forbidden("Owner permission required".to_string()));
    }

    state.db().organizations().delete(org.id).await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// List organization members.
async fn list_members(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<Vec<MemberResponse>>> {
    let org = state
        .db()
        .organizations()
        .get_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    // Check membership
    if !state.db().memberships().is_member(org.id, auth_user.user_id).await? {
        return Err(ApiError::Forbidden("Not a member of this organization".to_string()));
    }

    let members = state.db().memberships().list_members(org.id).await?;

    let response: Vec<MemberResponse> = members
        .into_iter()
        .map(|(user, membership)| MemberResponse {
            id: user.id.to_string(),
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            role: membership.role.to_string(),
        })
        .collect();

    Ok(Json(response))
}

/// Remove a member from organization.
async fn remove_member(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((slug, member_id)): Path<(String, String)>,
) -> ApiResult<Json<serde_json::Value>> {
    let org = state
        .db()
        .organizations()
        .get_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    // Check admin permission
    let membership = state
        .db()
        .memberships()
        .get(org.id, auth_user.user_id)
        .await?
        .ok_or_else(|| ApiError::Forbidden("Not a member of this organization".to_string()))?;

    if !membership.is_admin() {
        return Err(ApiError::Forbidden("Admin permission required".to_string()));
    }

    let member_uuid = Uuid::parse_str(&member_id)
        .map_err(|_| ApiError::BadRequest("Invalid member ID".to_string()))?;

    // Check if removing the last owner
    let target_membership = state
        .db()
        .memberships()
        .get(org.id, member_uuid)
        .await?
        .ok_or_else(|| ApiError::NotFound("Member not found".to_string()))?;

    if target_membership.is_owner() {
        let owner_count = state.db().memberships().owner_count(org.id).await?;
        if owner_count <= 1 {
            return Err(ApiError::BadRequest(
                "Cannot remove the last owner".to_string(),
            ));
        }
    }

    state.db().memberships().remove(org.id, member_uuid).await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Build organization routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_orgs).post(create_org))
        .route("/{slug}", get(get_org).patch(update_org).delete(delete_org))
        .route("/{slug}/members", get(list_members))
        .route("/{slug}/members/{member_id}", delete(remove_member))
        .nest("/{slug}/repos", super::repos::org_repo_routes())
}
