//! Authentication middleware and extractors.

use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

use super::jwt::JwtManager;

/// Authenticated user extractor.
#[derive(Debug, Clone)]
pub struct AuthUser {
    /// User ID
    pub user_id: Uuid,
    /// Session ID
    pub session_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // Extract authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| ApiError::Unauthorized("Missing authorization header".to_string()))?;

        // Extract bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| ApiError::Unauthorized("Invalid authorization format".to_string()))?;

        // Verify token
        let jwt_manager = JwtManager::new(
            &state.config().jwt_secret,
            state.config().jwt_expiry_days,
        );
        let claims = jwt_manager.verify_token(token)?;

        // Verify session is still valid
        let session_id = claims.session_id()?;
        let session = state
            .db()
            .sessions()
            .get_by_id(session_id)
            .await
            .map_err(|_| ApiError::Internal("Failed to verify session".to_string()))?
            .ok_or_else(|| ApiError::Unauthorized("Session not found".to_string()))?;

        if session.is_expired() {
            return Err(ApiError::Unauthorized("Session expired".to_string()));
        }

        Ok(AuthUser {
            user_id: claims.user_id()?,
            session_id,
        })
    }
}

/// Optional authenticated user extractor.
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl FromRequestParts<AppState> for OptionalAuthUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuthUser(Some(user))),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}
