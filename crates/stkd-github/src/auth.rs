//! GitHub authentication

use std::path::PathBuf;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// GitHub authentication methods
#[derive(Debug, Clone)]
pub enum GitHubAuth {
    /// Personal access token
    Token(String),
    /// OAuth token with expiry
    OAuth {
        access_token: String,
        refresh_token: Option<String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    },
}

impl GitHubAuth {
    /// Get the token for API requests
    pub fn token(&self) -> &str {
        match self {
            GitHubAuth::Token(t) => t,
            GitHubAuth::OAuth { access_token, .. } => access_token,
        }
    }

    /// Check if OAuth token needs refresh
    pub fn needs_refresh(&self) -> bool {
        if let GitHubAuth::OAuth { expires_at: Some(exp), .. } = self {
            *exp < chrono::Utc::now() + chrono::Duration::minutes(5)
        } else {
            false
        }
    }
}

/// Stored authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// Token type
    pub token_type: TokenType,
    /// The token value
    pub token: String,
    /// Refresh token (for OAuth)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// Expiry time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When this token was stored
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Type of token
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    /// Personal access token
    Pat,
    /// OAuth token
    OAuth,
}

impl AuthToken {
    /// Create a new PAT token
    pub fn pat(token: String) -> Self {
        Self {
            token_type: TokenType::Pat,
            token,
            refresh_token: None,
            expires_at: None,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create a new OAuth token
    pub fn oauth(token: String, refresh: Option<String>, expires_at: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        Self {
            token_type: TokenType::OAuth,
            token,
            refresh_token: refresh,
            expires_at,
            created_at: chrono::Utc::now(),
        }
    }

    /// Convert to GitHubAuth
    pub fn to_auth(&self) -> GitHubAuth {
        match self.token_type {
            TokenType::Pat => GitHubAuth::Token(self.token.clone()),
            TokenType::OAuth => GitHubAuth::OAuth {
                access_token: self.token.clone(),
                refresh_token: self.refresh_token.clone(),
                expires_at: self.expires_at,
            },
        }
    }
}

/// Path to stored credentials
pub fn credentials_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("stkd");

    std::fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join("github_token.json"))
}

/// Load stored credentials
pub fn load_credentials() -> Result<Option<AuthToken>> {
    let path = credentials_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)?;
    let token: AuthToken = serde_json::from_str(&content)?;

    Ok(Some(token))
}

/// Save credentials
pub fn save_credentials(token: &AuthToken) -> Result<()> {
    let path = credentials_path()?;
    let content = serde_json::to_string_pretty(token)?;

    std::fs::write(&path, content)?;

    // Set restrictive permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

/// Clear stored credentials
pub fn clear_credentials() -> Result<()> {
    let path = credentials_path()?;

    if path.exists() {
        std::fs::remove_file(&path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pat_token() {
        let token = AuthToken::pat("ghp_test123".to_string());
        assert_eq!(token.token_type, TokenType::Pat);
        assert!(token.expires_at.is_none());
    }

    #[test]
    fn test_oauth_token() {
        let token = AuthToken::oauth(
            "gho_test123".to_string(),
            Some("refresh_token".to_string()),
            Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        );
        assert_eq!(token.token_type, TokenType::OAuth);
        assert!(token.expires_at.is_some());
    }
}
