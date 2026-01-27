//! Server configuration.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use stkd_db::DatabaseConfig;

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
    ///
    /// # Required Environment Variables
    ///
    /// - `JWT_SECRET`: Secret key for JWT signing (required in production)
    ///
    /// # Optional Environment Variables
    ///
    /// - `HOST`: Server host (default: "0.0.0.0")
    /// - `PORT`: Server port (default: 3000)
    /// - `BASE_URL`: Public URL (default: "http://localhost:3000")
    /// - `JWT_EXPIRY_DAYS`: Token expiry in days (default: 7)
    /// - `STACK_DEV_MODE`: Set to "1" to allow insecure defaults for development
    pub fn from_env() -> Result<Self> {
        let database = DatabaseConfig::from_env()
            .context("Failed to load database configuration")?;

        let dev_mode = std::env::var("STACK_DEV_MODE").map(|v| v == "1").unwrap_or(false);

        let jwt_secret = match std::env::var("JWT_SECRET") {
            Ok(secret) => {
                if secret.len() < 32 {
                    anyhow::bail!(
                        "JWT_SECRET must be at least 32 characters for security. \
                        Current length: {}",
                        secret.len()
                    );
                }
                secret
            }
            Err(_) => {
                if dev_mode {
                    tracing::warn!(
                        "JWT_SECRET not set - using insecure default for development. \
                        DO NOT use in production!"
                    );
                    "insecure-dev-secret-do-not-use-in-production".to_string()
                } else {
                    anyhow::bail!(
                        "JWT_SECRET environment variable is required for security.\n\
                        Generate one with: openssl rand -base64 32\n\
                        For development only, set STACK_DEV_MODE=1 to use insecure defaults."
                    );
                }
            }
        };

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
