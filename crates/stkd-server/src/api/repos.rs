//! Repository API routes.

use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

/// Connect repository request.
#[derive(Debug, Deserialize)]
pub struct ConnectRepoRequest {
    provider: String,
    owner: String,
    name: String,
    default_branch: Option<String>,
}

/// Repository response.
#[derive(Debug, Serialize)]
pub struct RepoResponse {
    id: String,
    provider: String,
    owner: String,
    name: String,
    full_name: String,
    default_branch: String,
    is_active: bool,
    synced_at: Option<String>,
}

/// Stack response.
#[derive(Debug, Serialize)]
pub struct StackResponse {
    branches: Vec<BranchResponse>,
}

/// Branch response.
#[derive(Debug, Serialize)]
pub struct BranchResponse {
    id: String,
    name: String,
    parent_name: Option<String>,
    status: String,
    head_sha: Option<String>,
    mr: Option<MrResponse>,
}

/// Merge request response.
#[derive(Debug, Serialize)]
pub struct MrResponse {
    id: String,
    number: u64,
    title: String,
    state: String,
    url: String,
}

/// List repositories in an organization.
async fn list_repos(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
) -> ApiResult<Json<Vec<RepoResponse>>> {
    let org = state
        .db()
        .organizations()
        .get_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    // Check membership
    if !state
        .db()
        .memberships()
        .is_member(org.id, auth_user.user_id)
        .await?
    {
        return Err(ApiError::Forbidden(
            "Not a member of this organization".to_string(),
        ));
    }

    let repos = state.db().repositories().list_by_org(org.id).await?;

    let response: Vec<RepoResponse> = repos
        .into_iter()
        .map(|repo| RepoResponse {
            id: repo.id.to_string(),
            provider: repo.provider.clone(),
            owner: repo.owner.clone(),
            name: repo.name.clone(),
            full_name: repo.full_name(),
            default_branch: repo.default_branch,
            is_active: repo.is_active,
            synced_at: repo.synced_at.map(|t| t.to_rfc3339()),
        })
        .collect();

    Ok(Json(response))
}

/// Connect a repository to an organization.
async fn connect_repo(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(slug): Path<String>,
    Json(req): Json<ConnectRepoRequest>,
) -> ApiResult<Json<RepoResponse>> {
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

    // Check if repository is already connected
    if state
        .db()
        .repositories()
        .get_by_provider(&req.provider, &req.owner, &req.name)
        .await?
        .is_some()
    {
        return Err(ApiError::Conflict(
            "Repository already connected".to_string(),
        ));
    }

    // Create repository
    let provider_id = format!("{}:{}/{}", req.provider, req.owner, req.name);
    let repo = stkd_db::Repository::new(
        org.id,
        req.provider.clone(),
        req.owner.clone(),
        req.name.clone(),
        req.default_branch.unwrap_or_else(|| "main".to_string()),
        provider_id,
    );
    state.db().repositories().create(&repo).await?;

    let full_name = repo.full_name();
    Ok(Json(RepoResponse {
        id: repo.id.to_string(),
        provider: repo.provider,
        owner: repo.owner,
        name: repo.name,
        full_name,
        default_branch: repo.default_branch,
        is_active: repo.is_active,
        synced_at: None,
    }))
}

/// Disconnect a repository.
async fn disconnect_repo(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((slug, repo_id)): Path<(String, String)>,
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

    let repo_uuid = Uuid::parse_str(&repo_id)
        .map_err(|_| ApiError::BadRequest("Invalid repository ID".to_string()))?;

    state.db().repositories().delete(repo_uuid).await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Get repository stacks (branches with their relationships).
async fn get_stacks(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(repo_id): Path<String>,
) -> ApiResult<Json<StackResponse>> {
    let repo_uuid = Uuid::parse_str(&repo_id)
        .map_err(|_| ApiError::BadRequest("Invalid repository ID".to_string()))?;

    let repo = state
        .db()
        .repositories()
        .get_by_id(repo_uuid)
        .await?
        .ok_or_else(|| ApiError::NotFound("Repository not found".to_string()))?;

    // Check membership
    if !state
        .db()
        .memberships()
        .is_member(repo.org_id, auth_user.user_id)
        .await?
    {
        return Err(ApiError::Forbidden(
            "Not a member of this organization".to_string(),
        ));
    }

    // Get all branches
    let branches = state.db().branches().list_by_repo(repo_uuid).await?;

    // Get merge requests for each branch
    let mut branch_responses = Vec::new();
    for branch in branches {
        let mr = if let Some(mr_id) = branch.mr_id {
            state
                .db()
                .merge_requests()
                .get_by_id(mr_id)
                .await?
                .map(|mr| MrResponse {
                    id: mr.id.to_string(),
                    number: mr.number,
                    title: mr.title,
                    state: mr.state.to_string(),
                    url: mr.url,
                })
        } else {
            None
        };

        branch_responses.push(BranchResponse {
            id: branch.id.to_string(),
            name: branch.name,
            parent_name: branch.parent_name,
            status: branch.status.to_string(),
            head_sha: branch.head_sha,
            mr,
        });
    }

    Ok(Json(StackResponse {
        branches: branch_responses,
    }))
}

/// Trigger repository sync.
async fn sync_repo(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((slug, repo_id)): Path<(String, String)>,
) -> ApiResult<Json<serde_json::Value>> {
    let org = state
        .db()
        .organizations()
        .get_by_slug(&slug)
        .await?
        .ok_or_else(|| ApiError::NotFound("Organization not found".to_string()))?;

    // Check membership
    if !state
        .db()
        .memberships()
        .is_member(org.id, auth_user.user_id)
        .await?
    {
        return Err(ApiError::Forbidden(
            "Not a member of this organization".to_string(),
        ));
    }

    let repo_uuid = Uuid::parse_str(&repo_id)
        .map_err(|_| ApiError::BadRequest("Invalid repository ID".to_string()))?;

    // Mark as synced (actual sync would be done by a background job)
    state.db().repositories().mark_synced(repo_uuid).await?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Build repository routes.
/// These routes are nested under /api/repos
pub fn routes() -> Router<AppState> {
    Router::new().route("/{repo_id}/stacks", get(get_stacks))
}

/// Organization repository routes (nested under /api/orgs/:slug)
pub fn org_repo_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_repos).post(connect_repo))
        .route("/{repo_id}", delete(disconnect_repo))
        .route("/{repo_id}/sync", post(sync_repo))
}
