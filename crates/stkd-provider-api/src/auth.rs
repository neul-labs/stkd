//! Authentication abstractions for Git hosting providers.
//!
//! This module provides a unified interface for managing credentials
//! across different Git hosting providers.
//!
//! # Credential Types
//!
//! - **Personal Access Token (PAT)**: Static token with defined scopes
//! - **OAuth2**: Token with optional refresh capability
//! - **Job Token**: CI/CD-specific tokens (GitLab)
//! - **Deploy Token**: Read-only access tokens
//!
//! # Storage
//!
//! Credentials are stored in the user's config directory:
//! - Linux: `~/.config/stack/credentials/`
//! - macOS: `~/Library/Application Support/stack/credentials/`
//! - Windows: `%APPDATA%\stack\credentials\`

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

use crate::error::{ProviderError, ProviderResult};

/// Authentication credential types.
///
/// Represents the different authentication methods supported across providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Credential {
    /// Personal Access Token (GitHub PAT, GitLab PAT, etc.)
    PersonalAccessToken {
        /// The token value
        token: String,
        /// Optional expiration time
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_at: Option<DateTime<Utc>>,
    },

    /// OAuth2 token with optional refresh capability
    OAuth2 {
        /// Access token for API requests
        access_token: String,
        /// Refresh token for obtaining new access tokens
        #[serde(skip_serializing_if = "Option::is_none")]
        refresh_token: Option<String>,
        /// When the access token expires
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_at: Option<DateTime<Utc>>,
        /// Token scopes
        #[serde(skip_serializing_if = "Option::is_none")]
        scopes: Option<Vec<String>>,
    },

    /// CI/CD job token (GitLab)
    JobToken {
        /// The job token
        token: String,
    },

    /// Deploy token for read-only access
    DeployToken {
        /// Username for the deploy token
        username: String,
        /// Token value
        token: String,
    },
}

impl Credential {
    /// Create a new Personal Access Token credential.
    pub fn pat(token: impl Into<String>) -> Self {
        Self::PersonalAccessToken {
            token: token.into(),
            expires_at: None,
        }
    }

    /// Create a new OAuth2 credential.
    pub fn oauth2(access_token: impl Into<String>) -> Self {
        Self::OAuth2 {
            access_token: access_token.into(),
            refresh_token: None,
            expires_at: None,
            scopes: None,
        }
    }

    /// Get the token value for API requests.
    ///
    /// Returns the access token, PAT, or job token depending on the type.
    pub fn token(&self) -> &str {
        match self {
            Credential::PersonalAccessToken { token, .. } => token,
            Credential::OAuth2 { access_token, .. } => access_token,
            Credential::JobToken { token } => token,
            Credential::DeployToken { token, .. } => token,
        }
    }

    /// Check if the credential needs to be refreshed.
    ///
    /// Returns `true` if:
    /// - It's an OAuth2 token with a refresh token that expires within 5 minutes
    pub fn needs_refresh(&self) -> bool {
        match self {
            Credential::OAuth2 {
                expires_at: Some(exp),
                refresh_token: Some(_),
                ..
            } => *exp < Utc::now() + chrono::Duration::minutes(5),
            _ => false,
        }
    }

    /// Check if the credential is expired (cannot be refreshed).
    ///
    /// Returns `true` if the token has expired and cannot be refreshed.
    pub fn is_expired(&self) -> bool {
        match self {
            Credential::PersonalAccessToken {
                expires_at: Some(exp),
                ..
            } => *exp < Utc::now(),
            Credential::OAuth2 {
                expires_at: Some(exp),
                refresh_token: None,
                ..
            } => *exp < Utc::now(),
            _ => false,
        }
    }

    /// Check if this is an OAuth2 credential that can be refreshed.
    pub fn can_refresh(&self) -> bool {
        matches!(
            self,
            Credential::OAuth2 {
                refresh_token: Some(_),
                ..
            }
        )
    }
}

/// Stored credential with metadata.
///
/// This wraps a credential with additional context about
/// where it came from and who authenticated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCredential {
    /// Provider name (github, gitlab, gitea)
    pub provider: String,

    /// Host (e.g., "github.com", "gitlab.mycompany.com")
    pub host: String,

    /// The actual credential
    pub credential: Credential,

    /// When this credential was stored
    pub created_at: DateTime<Utc>,

    /// Username of the authenticated user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl StoredCredential {
    /// Create a new stored credential.
    pub fn new(
        provider: impl Into<String>,
        host: impl Into<String>,
        credential: Credential,
    ) -> Self {
        Self {
            provider: provider.into(),
            host: host.into(),
            credential,
            created_at: Utc::now(),
            username: None,
        }
    }

    /// Set the username.
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
}

/// Trait for credential storage backends.
///
/// Implementations handle the secure storage of credentials.
pub trait CredentialStore: Send + Sync {
    /// Get the path to credentials file for a provider/host.
    fn credentials_path(&self, provider: &str, host: &str) -> io::Result<PathBuf>;

    /// Load a stored credential.
    fn load(&self, provider: &str, host: &str) -> io::Result<Option<StoredCredential>>;

    /// Save a credential.
    fn save(&self, credential: &StoredCredential) -> io::Result<()>;

    /// Clear (delete) a credential.
    fn clear(&self, provider: &str, host: &str) -> io::Result<()>;

    /// List all stored credentials.
    fn list(&self) -> io::Result<Vec<StoredCredential>>;
}

/// File-based credential store.
///
/// Stores credentials as JSON files in the config directory.
/// Files are created with restrictive permissions (0600 on Unix).
pub struct FileCredentialStore {
    base_dir: PathBuf,
}

impl FileCredentialStore {
    /// Create a new file credential store.
    ///
    /// # Errors
    ///
    /// Returns an error if the config directory cannot be determined or created.
    pub fn new() -> io::Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?
            .join("stkd")
            .join("credentials");

        std::fs::create_dir_all(&config_dir)?;

        Ok(Self {
            base_dir: config_dir,
        })
    }

    /// Create a file credential store with a custom base directory.
    pub fn with_base_dir(base_dir: PathBuf) -> io::Result<Self> {
        std::fs::create_dir_all(&base_dir)?;
        Ok(Self { base_dir })
    }

    /// Sanitize a host string for use in a filename.
    fn sanitize_host(host: &str) -> String {
        host.replace(['/', ':', '.'], "_")
    }
}

impl CredentialStore for FileCredentialStore {
    fn credentials_path(&self, provider: &str, host: &str) -> io::Result<PathBuf> {
        let safe_host = Self::sanitize_host(host);
        Ok(self.base_dir.join(format!("{}_{}.json", provider, safe_host)))
    }

    fn load(&self, provider: &str, host: &str) -> io::Result<Option<StoredCredential>> {
        let path = self.credentials_path(provider, host)?;

        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)?;
        let cred: StoredCredential = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Some(cred))
    }

    fn save(&self, credential: &StoredCredential) -> io::Result<()> {
        let path = self.credentials_path(&credential.provider, &credential.host)?;

        let content = serde_json::to_string_pretty(credential)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        std::fs::write(&path, content)?;

        // Set restrictive permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    fn clear(&self, provider: &str, host: &str) -> io::Result<()> {
        let path = self.credentials_path(provider, host)?;

        if path.exists() {
            std::fs::remove_file(&path)?;
        }

        Ok(())
    }

    fn list(&self) -> io::Result<Vec<StoredCredential>> {
        let mut credentials = Vec::new();

        if !self.base_dir.exists() {
            return Ok(credentials);
        }

        for entry in std::fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_some_and(|e| e == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(cred) = serde_json::from_str(&content) {
                        credentials.push(cred);
                    }
                }
            }
        }

        Ok(credentials)
    }
}

/// Helper function to load credentials for a provider.
///
/// This is a convenience function that uses the default file store.
pub fn load_credentials(provider: &str, host: &str) -> ProviderResult<Option<StoredCredential>> {
    let store = FileCredentialStore::new()
        .map_err(|e| ProviderError::Internal(format!("Failed to init credential store: {}", e)))?;

    store
        .load(provider, host)
        .map_err(|e| ProviderError::Internal(format!("Failed to load credentials: {}", e)))
}

/// Helper function to save credentials for a provider.
///
/// This is a convenience function that uses the default file store.
pub fn save_credentials(credential: &StoredCredential) -> ProviderResult<()> {
    let store = FileCredentialStore::new()
        .map_err(|e| ProviderError::Internal(format!("Failed to init credential store: {}", e)))?;

    store
        .save(credential)
        .map_err(|e| ProviderError::Internal(format!("Failed to save credentials: {}", e)))
}

/// Helper function to clear credentials for a provider.
///
/// This is a convenience function that uses the default file store.
pub fn clear_credentials(provider: &str, host: &str) -> ProviderResult<()> {
    let store = FileCredentialStore::new()
        .map_err(|e| ProviderError::Internal(format!("Failed to init credential store: {}", e)))?;

    store
        .clear(provider, host)
        .map_err(|e| ProviderError::Internal(format!("Failed to clear credentials: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_credential_pat() {
        let cred = Credential::pat("ghp_xxxx");
        assert_eq!(cred.token(), "ghp_xxxx");
        assert!(!cred.needs_refresh());
        assert!(!cred.is_expired());
    }

    #[test]
    fn test_credential_oauth2() {
        let cred = Credential::oauth2("access_token_123");
        assert_eq!(cred.token(), "access_token_123");
        assert!(!cred.can_refresh());
    }

    #[test]
    fn test_file_credential_store() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        let store = FileCredentialStore::with_base_dir(temp_dir.path().to_path_buf())?;

        // Test save and load
        let cred = StoredCredential::new("github", "github.com", Credential::pat("test_token"))
            .with_username("testuser");

        store.save(&cred)?;

        let loaded = store.load("github", "github.com")?;
        assert!(loaded.is_some());

        let loaded = loaded.unwrap();
        assert_eq!(loaded.provider, "github");
        assert_eq!(loaded.host, "github.com");
        assert_eq!(loaded.credential.token(), "test_token");
        assert_eq!(loaded.username.as_deref(), Some("testuser"));

        // Test list
        let all = store.list()?;
        assert_eq!(all.len(), 1);

        // Test clear
        store.clear("github", "github.com")?;
        let loaded = store.load("github", "github.com")?;
        assert!(loaded.is_none());

        Ok(())
    }

    #[test]
    fn test_sanitize_host() {
        assert_eq!(
            FileCredentialStore::sanitize_host("github.com"),
            "github_com"
        );
        assert_eq!(
            FileCredentialStore::sanitize_host("gitlab.mycompany.com"),
            "gitlab_mycompany_com"
        );
    }
}
