//! OAuth authentication providers.

pub mod github;
pub mod gitlab;

use serde::{Deserialize, Serialize};

/// OAuth user info returned by providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    /// Provider-specific user ID
    pub provider_id: String,
    /// Username
    pub username: String,
    /// Email (if available)
    pub email: Option<String>,
    /// Display name (if available)
    pub display_name: Option<String>,
    /// Avatar URL (if available)
    pub avatar_url: Option<String>,
}

/// OAuth tokens returned after authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    /// Access token
    pub access_token: String,
    /// Refresh token (if available)
    pub refresh_token: Option<String>,
    /// Token expiry in seconds (if available)
    pub expires_in: Option<u64>,
}
