//! GitLab OAuth provider.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::{OAuthTokens, OAuthUserInfo};
use crate::config::OAuthProviderConfig;

/// GitLab OAuth provider.
pub struct GitLabOAuth {
    config: OAuthProviderConfig,
    redirect_uri: String,
    host: String,
}

impl GitLabOAuth {
    /// Create a new GitLab OAuth provider.
    pub fn new(config: OAuthProviderConfig, redirect_uri: String) -> Self {
        let host = config
            .host
            .clone()
            .unwrap_or_else(|| "gitlab.com".to_string());
        Self {
            config,
            redirect_uri,
            host,
        }
    }

    fn auth_url(&self) -> String {
        format!("https://{}/oauth/authorize", self.host)
    }

    fn token_url(&self) -> String {
        format!("https://{}/oauth/token", self.host)
    }

    fn user_url(&self) -> String {
        format!("https://{}/api/v4/user", self.host)
    }

    /// Get the authorization URL.
    pub fn authorization_url(&self, state: &str) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=read_user&state={}",
            self.auth_url(),
            self.config.client_id,
            urlencoding::encode(&self.redirect_uri),
            state
        )
    }

    /// Exchange authorization code for tokens.
    pub async fn exchange_code(&self, code: &str) -> Result<OAuthTokens> {
        #[derive(Serialize)]
        struct TokenRequest {
            client_id: String,
            client_secret: String,
            code: String,
            redirect_uri: String,
            grant_type: String,
        }

        #[derive(Deserialize)]
        #[allow(dead_code)] // Fields required for deserialization
        struct TokenResponse {
            access_token: String,
            refresh_token: Option<String>,
            expires_in: Option<u64>,
            token_type: String,
        }

        let client = reqwest::Client::new();
        let response: TokenResponse = client
            .post(self.token_url())
            .json(&TokenRequest {
                client_id: self.config.client_id.clone(),
                client_secret: self.config.client_secret.clone(),
                code: code.to_string(),
                redirect_uri: self.redirect_uri.clone(),
                grant_type: "authorization_code".to_string(),
            })
            .send()
            .await
            .context("Failed to send token request")?
            .json()
            .await
            .context("Failed to parse token response")?;

        Ok(OAuthTokens {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_in: response.expires_in,
        })
    }

    /// Get user info from access token.
    pub async fn get_user_info(&self, access_token: &str) -> Result<OAuthUserInfo> {
        #[derive(Deserialize)]
        struct GitLabUser {
            id: u64,
            username: String,
            email: Option<String>,
            name: Option<String>,
            avatar_url: Option<String>,
        }

        let client = reqwest::Client::new();
        let user: GitLabUser = client
            .get(self.user_url())
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .context("Failed to get user info")?
            .json()
            .await
            .context("Failed to parse user info")?;

        Ok(OAuthUserInfo {
            provider_id: user.id.to_string(),
            username: user.username,
            email: user.email,
            display_name: user.name,
            avatar_url: user.avatar_url,
        })
    }
}
