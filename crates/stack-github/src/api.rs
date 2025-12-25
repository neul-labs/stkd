//! GitHub API client

use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{de::DeserializeOwned, Serialize};

use crate::auth::GitHubAuth;

/// GitHub API client
pub struct GitHubClient {
    client: reqwest::Client,
    base_url: String,
    auth: GitHubAuth,
}

impl GitHubClient {
    /// Create a new client with authentication
    pub fn new(auth: GitHubAuth) -> Result<Self> {
        Self::with_base_url(auth, "https://api.github.com")
    }

    /// Create a client with custom base URL (for GitHub Enterprise)
    pub fn with_base_url(auth: GitHubAuth, base_url: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github+json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("stack-cli/0.1.0"));
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            auth,
        })
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, text);
        }

        response.json().await.context("Failed to parse response")
    }

    /// Make a POST request
    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, text);
        }

        response.json().await.context("Failed to parse response")
    }

    /// Make a PATCH request
    pub async fn patch<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .patch(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, text);
        }

        response.json().await.context("Failed to parse response")
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, text);
        }

        Ok(())
    }

    /// Get current user info
    pub async fn current_user(&self) -> Result<User> {
        self.get("/user").await
    }

    /// Check if token is valid
    pub async fn validate_token(&self) -> Result<bool> {
        match self.current_user().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// GitHub user info
#[derive(Debug, Clone, serde::Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let auth = GitHubAuth::Token("test_token".to_string());
        let client = GitHubClient::new(auth);
        assert!(client.is_ok());
    }
}
