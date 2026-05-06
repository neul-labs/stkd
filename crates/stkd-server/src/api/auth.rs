//! Authentication API routes.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::oauth::{github::GitHubOAuth, gitlab::GitLabOAuth};
use crate::auth::{AuthUser, JwtManager};
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

/// OAuth callback query parameters.
#[derive(Debug, Deserialize)]
pub struct OAuthCallback {
    code: String,
    state: String,
}

/// OAuth start response.
#[derive(Debug, Serialize)]
pub struct OAuthStartResponse {
    url: String,
    state: String,
}

/// Login response.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    user: UserResponse,
}

/// User response.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    id: String,
    username: String,
    email: Option<String>,
    display_name: Option<String>,
    avatar_url: Option<String>,
    provider: String,
}

/// Start OAuth flow.
async fn oauth_start(
    State(state): State<AppState>,
    Path(provider): Path<String>,
) -> ApiResult<Json<OAuthStartResponse>> {
    let oauth_state = Uuid::new_v4().to_string();
    let redirect_uri = format!(
        "{}/api/auth/oauth/{}/callback",
        state.config().base_url,
        provider
    );

    let url =
        match provider.as_str() {
            "github" => {
                let config = state.config().oauth.github.as_ref().ok_or_else(|| {
                    ApiError::BadRequest("GitHub OAuth not configured".to_string())
                })?;
                let oauth = GitHubOAuth::new(config.clone(), redirect_uri);
                oauth.authorization_url(&oauth_state)
            }
            "gitlab" => {
                let config = state.config().oauth.gitlab.as_ref().ok_or_else(|| {
                    ApiError::BadRequest("GitLab OAuth not configured".to_string())
                })?;
                let oauth = GitLabOAuth::new(config.clone(), redirect_uri);
                oauth.authorization_url(&oauth_state)
            }
            _ => {
                return Err(ApiError::BadRequest(format!(
                    "Unknown provider: {}",
                    provider
                )))
            }
        };

    // Store state for CSRF validation
    state.oauth_states().store(&oauth_state);

    Ok(Json(OAuthStartResponse {
        url,
        state: oauth_state,
    }))
}

/// Handle OAuth callback.
async fn oauth_callback(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    Query(callback): Query<OAuthCallback>,
) -> ApiResult<Json<LoginResponse>> {
    // Validate OAuth state to prevent CSRF attacks
    if !state.oauth_states().validate(&callback.state) {
        return Err(ApiError::BadRequest(
            "Invalid or expired OAuth state. Please try logging in again.".to_string(),
        ));
    }

    let redirect_uri = format!(
        "{}/api/auth/oauth/{}/callback",
        state.config().base_url,
        provider
    );

    // Exchange code for tokens and get user info
    let user_info =
        match provider.as_str() {
            "github" => {
                let config = state.config().oauth.github.as_ref().ok_or_else(|| {
                    ApiError::BadRequest("GitHub OAuth not configured".to_string())
                })?;
                let oauth = GitHubOAuth::new(config.clone(), redirect_uri);
                let tokens = oauth
                    .exchange_code(&callback.code)
                    .await
                    .map_err(|e| ApiError::Internal(e.to_string()))?;
                oauth
                    .get_user_info(&tokens.access_token)
                    .await
                    .map_err(|e| ApiError::Internal(e.to_string()))?
            }
            "gitlab" => {
                let config = state.config().oauth.gitlab.as_ref().ok_or_else(|| {
                    ApiError::BadRequest("GitLab OAuth not configured".to_string())
                })?;
                let oauth = GitLabOAuth::new(config.clone(), redirect_uri);
                let tokens = oauth
                    .exchange_code(&callback.code)
                    .await
                    .map_err(|e| ApiError::Internal(e.to_string()))?;
                oauth
                    .get_user_info(&tokens.access_token)
                    .await
                    .map_err(|e| ApiError::Internal(e.to_string()))?
            }
            _ => {
                return Err(ApiError::BadRequest(format!(
                    "Unknown provider: {}",
                    provider
                )))
            }
        };

    // Find or create user
    let user = state
        .db()
        .users()
        .find_or_create_by_oauth(
            &provider,
            &user_info.provider_id,
            &user_info.username,
            user_info.email.as_deref(),
            user_info.display_name.as_deref(),
            user_info.avatar_url.as_deref(),
        )
        .await?;

    // Create session
    let session_id = Uuid::new_v4();
    let token_hash = format!("{:x}", md5::compute(session_id.to_string()));
    let session = stkd_db::Session::new(user.id, token_hash, state.config().jwt_expiry_days);
    state.db().sessions().create(&session).await?;

    // Create JWT token
    let jwt_manager = JwtManager::new(&state.config().jwt_secret, state.config().jwt_expiry_days);
    let token = jwt_manager.create_token(user.id, session.id)?;

    Ok(Json(LoginResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            provider: user.provider,
        },
    }))
}

/// Get current user.
async fn me(State(state): State<AppState>, auth_user: AuthUser) -> ApiResult<Json<UserResponse>> {
    let user = state
        .db()
        .users()
        .get_by_id(auth_user.user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        display_name: user.display_name,
        avatar_url: user.avatar_url,
        provider: user.provider,
    }))
}

/// Logout.
async fn logout(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> ApiResult<Json<serde_json::Value>> {
    state.db().sessions().delete(auth_user.session_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

/// Build auth routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/oauth/{provider}/start", post(oauth_start))
        .route("/oauth/{provider}/callback", get(oauth_callback))
        .route("/me", get(me))
        .route("/logout", post(logout))
}
