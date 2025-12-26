//! GitLab authentication
//!
//! Supports:
//! - Personal Access Tokens (PAT)
//! - OAuth tokens
//! - Job tokens (for CI/CD)

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// GitLab authentication method
#[derive(Debug, Clone)]
pub enum GitLabAuth {
    /// Personal access token
    PersonalAccessToken(String),
    /// OAuth2 token
    OAuth(String),
    /// CI/CD job token
    JobToken(String),
}

impl GitLabAuth {
    /// Get the token value
    pub fn token(&self) -> &str {
        match self {
            GitLabAuth::PersonalAccessToken(t) => t,
            GitLabAuth::OAuth(t) => t,
            GitLabAuth::JobToken(t) => t,
        }
    }

    /// Get the header name for this auth type
    pub fn header_name(&self) -> &'static str {
        match self {
            GitLabAuth::PersonalAccessToken(_) => "PRIVATE-TOKEN",
            GitLabAuth::OAuth(_) => "Authorization",
            GitLabAuth::JobToken(_) => "JOB-TOKEN",
        }
    }

    /// Get the header value for this auth type
    pub fn header_value(&self) -> String {
        match self {
            GitLabAuth::PersonalAccessToken(t) => t.clone(),
            GitLabAuth::OAuth(t) => format!("Bearer {}", t),
            GitLabAuth::JobToken(t) => t.clone(),
        }
    }
}

/// Stored authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// The token value
    pub token: String,
    /// Token type
    pub token_type: String,
    /// GitLab host (e.g., "gitlab.com")
    pub host: String,
    /// When the token expires (if known)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl AuthToken {
    /// Create a new PAT token
    pub fn pat(token: impl Into<String>, host: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            token_type: "pat".to_string(),
            host: host.into(),
            expires_at: None,
        }
    }

    /// Convert to GitLabAuth
    pub fn to_auth(&self) -> GitLabAuth {
        match self.token_type.as_str() {
            "oauth" => GitLabAuth::OAuth(self.token.clone()),
            "job" => GitLabAuth::JobToken(self.token.clone()),
            _ => GitLabAuth::PersonalAccessToken(self.token.clone()),
        }
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| exp < chrono::Utc::now())
            .unwrap_or(false)
    }
}

/// Get the credentials file path
pub fn credentials_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("stack")
        .join("gitlab_credentials.json")
}

/// Load stored credentials for a host
pub fn load_credentials(host: &str) -> anyhow::Result<Option<AuthToken>> {
    let path = credentials_path();
    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)?;
    let tokens: std::collections::HashMap<String, AuthToken> = serde_json::from_str(&content)?;

    Ok(tokens.get(host).cloned())
}

/// Save credentials for a host
pub fn save_credentials(token: &AuthToken) -> anyhow::Result<()> {
    let path = credentials_path();

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Load existing tokens
    let mut tokens: std::collections::HashMap<String, AuthToken> = if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        std::collections::HashMap::new()
    };

    // Add/update token
    tokens.insert(token.host.clone(), token.clone());

    // Save
    let content = serde_json::to_string_pretty(&tokens)?;
    std::fs::write(&path, content)?;

    Ok(())
}

/// Remove credentials for a host
pub fn remove_credentials(host: &str) -> anyhow::Result<()> {
    let path = credentials_path();
    if !path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&path)?;
    let mut tokens: std::collections::HashMap<String, AuthToken> = serde_json::from_str(&content)?;

    tokens.remove(host);

    let content = serde_json::to_string_pretty(&tokens)?;
    std::fs::write(&path, content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_header() {
        let pat = GitLabAuth::PersonalAccessToken("token123".to_string());
        assert_eq!(pat.header_name(), "PRIVATE-TOKEN");
        assert_eq!(pat.header_value(), "token123");

        let oauth = GitLabAuth::OAuth("oauth123".to_string());
        assert_eq!(oauth.header_name(), "Authorization");
        assert_eq!(oauth.header_value(), "Bearer oauth123");
    }
}
