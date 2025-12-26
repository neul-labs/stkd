//! Server configuration.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use stack_db::DatabaseConfig;

/// Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Base URL for the server
    pub base_url: String,
    /// Database configuration
    pub database: DatabaseConfig,
    /// JWT secret for authentication
    pub jwt_secret: String,
    /// JWT expiry in days
    pub jwt_expiry_days: i64,
    /// OAuth configuration
    pub oauth: OAuthConfig,
}

/// OAuth provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// GitHub OAuth settings
    pub github: Option<OAuthProviderConfig>,
    /// GitLab OAuth settings
    pub gitlab: Option<OAuthProviderConfig>,
}

/// OAuth provider-specific configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProviderConfig {
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Optional custom host (for self-hosted instances)
    pub host: Option<String>,
}

impl ServerConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self> {
        let database = DatabaseConfig::from_env()
            .context("Failed to load database configuration")?;

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                tracing::warn!("JWT_SECRET not set, using insecure default");
                "insecure-default-jwt-secret-change-in-production".to_string()
            });

        let github = match (
            std::env::var("GITHUB_CLIENT_ID"),
            std::env::var("GITHUB_CLIENT_SECRET"),
        ) {
            (Ok(client_id), Ok(client_secret)) => Some(OAuthProviderConfig {
                client_id,
                client_secret,
                host: None,
            }),
            _ => None,
        };

        let gitlab = match (
            std::env::var("GITLAB_CLIENT_ID"),
            std::env::var("GITLAB_CLIENT_SECRET"),
        ) {
            (Ok(client_id), Ok(client_secret)) => Some(OAuthProviderConfig {
                client_id,
                client_secret,
                host: std::env::var("GITLAB_HOST").ok(),
            }),
            _ => None,
        };

        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            base_url: std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            database,
            jwt_secret,
            jwt_expiry_days: std::env::var("JWT_EXPIRY_DAYS")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(7),
            oauth: OAuthConfig { github, gitlab },
        })
    }

    /// Create a test configuration.
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 0,
            base_url: "http://localhost:3000".to_string(),
            database: DatabaseConfig::sqlite_memory(),
            jwt_secret: "test-secret".to_string(),
            jwt_expiry_days: 1,
            oauth: OAuthConfig {
                github: None,
                gitlab: None,
            },
        }
    }
}
