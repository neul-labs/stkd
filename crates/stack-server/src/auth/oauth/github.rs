//! GitHub OAuth provider.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::{OAuthTokens, OAuthUserInfo};
use crate::config::OAuthProviderConfig;

const GITHUB_AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_USER_URL: &str = "https://api.github.com/user";

/// GitHub OAuth provider.
pub struct GitHubOAuth {
    config: OAuthProviderConfig,
    redirect_uri: String,
}

impl GitHubOAuth {
    /// Create a new GitHub OAuth provider.
    pub fn new(config: OAuthProviderConfig, redirect_uri: String) -> Self {
        Self { config, redirect_uri }
    }

    /// Get the authorization URL.
    pub fn authorization_url(&self, state: &str) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&scope=read:user,user:email&state={}",
            GITHUB_AUTH_URL,
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
        }

        #[derive(Deserialize)]
        #[allow(dead_code)] // Fields required for deserialization
        struct TokenResponse {
            access_token: String,
            token_type: String,
            scope: String,
        }

        let client = reqwest::Client::new();
        let response: TokenResponse = client
            .post(GITHUB_TOKEN_URL)
            .header("Accept", "application/json")
            .json(&TokenRequest {
                client_id: self.config.client_id.clone(),
                client_secret: self.config.client_secret.clone(),
                code: code.to_string(),
                redirect_uri: self.redirect_uri.clone(),
            })
            .send()
            .await
            .context("Failed to send token request")?
            .json()
            .await
            .context("Failed to parse token response")?;

        Ok(OAuthTokens {
            access_token: response.access_token,
            refresh_token: None,
            expires_in: None,
        })
    }

    /// Get user info from access token.
    pub async fn get_user_info(&self, access_token: &str) -> Result<OAuthUserInfo> {
        #[derive(Deserialize)]
        struct GitHubUser {
            id: u64,
            login: String,
            email: Option<String>,
            name: Option<String>,
            avatar_url: Option<String>,
        }

        let client = reqwest::Client::new();
        let user: GitHubUser = client
            .get(GITHUB_USER_URL)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "Stack-Server")
            .send()
            .await
            .context("Failed to get user info")?
            .json()
            .await
            .context("Failed to parse user info")?;

        Ok(OAuthUserInfo {
            provider_id: user.id.to_string(),
            username: user.login,
            email: user.email,
            display_name: user.name,
            avatar_url: user.avatar_url,
        })
    }
}
