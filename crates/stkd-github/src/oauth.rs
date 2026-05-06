//! GitHub OAuth Device Flow
//!
//! Implements the OAuth 2.0 Device Authorization Grant for GitHub.
//! This allows users to authenticate without manually copying tokens.
//!
//! See: https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps#device-flow

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// GitHub OAuth App Client ID for Stack CLI
/// This should be registered at https://github.com/settings/developers
/// For now, users should create their own OAuth App or use PAT
pub const DEFAULT_CLIENT_ID: &str = ""; // Users must provide their own

/// Device code request response
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeResponse {
    /// The device verification code
    pub device_code: String,
    /// The user verification code to enter
    pub user_code: String,
    /// The URL where user should enter the code
    pub verification_uri: String,
    /// Seconds until codes expire
    pub expires_in: u64,
    /// Seconds to wait between polling
    pub interval: u64,
}

/// Token response from GitHub
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    /// The access token
    pub access_token: String,
    /// Token type (usually "bearer")
    pub token_type: String,
    /// OAuth scopes granted
    pub scope: String,
}

/// Error response from token endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct TokenError {
    pub error: String,
    pub error_description: Option<String>,
}

/// OAuth device flow client
pub struct DeviceFlow {
    client: reqwest::Client,
    client_id: String,
}

impl DeviceFlow {
    /// Create a new device flow client
    pub fn new(client_id: impl Into<String>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            client_id: client_id.into(),
        })
    }

    /// Start the device flow by requesting a device code
    pub async fn request_device_code(&self, scope: &str) -> Result<DeviceCodeResponse> {
        #[derive(Serialize)]
        struct Request<'a> {
            client_id: &'a str,
            scope: &'a str,
        }

        let response = self
            .client
            .post("https://github.com/login/device/code")
            .header("Accept", "application/json")
            .form(&Request {
                client_id: &self.client_id,
                scope,
            })
            .send()
            .await
            .context("Failed to request device code")?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to request device code: {}", text);
        }

        response
            .json()
            .await
            .context("Failed to parse device code response")
    }

    /// Poll for the access token
    /// Returns None if the user hasn't authorized yet, Err if there's an error
    pub async fn poll_for_token(&self, device_code: &str) -> Result<Option<TokenResponse>> {
        #[derive(Serialize)]
        struct Request<'a> {
            client_id: &'a str,
            device_code: &'a str,
            grant_type: &'a str,
        }

        let response = self
            .client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&Request {
                client_id: &self.client_id,
                device_code,
                grant_type: "urn:ietf:params:oauth:grant-type:device_code",
            })
            .send()
            .await
            .context("Failed to poll for token")?;

        let text = response.text().await?;

        // Try to parse as success response
        if let Ok(token) = serde_json::from_str::<TokenResponse>(&text) {
            return Ok(Some(token));
        }

        // Try to parse as error response
        if let Ok(error) = serde_json::from_str::<TokenError>(&text) {
            match error.error.as_str() {
                "authorization_pending" => {
                    // User hasn't authorized yet, keep polling
                    return Ok(None);
                }
                "slow_down" => {
                    // We're polling too fast, caller should increase interval
                    return Ok(None);
                }
                "expired_token" => {
                    anyhow::bail!("Device code expired. Please try again.");
                }
                "access_denied" => {
                    anyhow::bail!("Authorization was denied by user.");
                }
                _ => {
                    anyhow::bail!(
                        "OAuth error: {} - {}",
                        error.error,
                        error.error_description.unwrap_or_default()
                    );
                }
            }
        }

        anyhow::bail!("Unexpected response from GitHub: {}", text);
    }

    /// Run the complete device flow with polling
    pub async fn authenticate(&self, scope: &str) -> Result<TokenResponse> {
        let device_code = self.request_device_code(scope).await?;

        println!("\nTo authenticate, visit: {}", device_code.verification_uri);
        println!("And enter code: {}\n", device_code.user_code);

        // Try to open the browser
        let _ = open_browser(&device_code.verification_uri);

        println!("Waiting for authorization...");

        let interval = Duration::from_secs(device_code.interval.max(5));
        let deadline = std::time::Instant::now() + Duration::from_secs(device_code.expires_in);

        loop {
            if std::time::Instant::now() > deadline {
                anyhow::bail!("Device code expired. Please try again.");
            }

            tokio::time::sleep(interval).await;

            match self.poll_for_token(&device_code.device_code).await? {
                Some(token) => return Ok(token),
                None => continue,
            }
        }
    }
}

/// Try to open a URL in the default browser
fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .context("Failed to open browser")?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .context("Failed to open browser")?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn()
            .context("Failed to open browser")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_flow_creation() {
        let flow = DeviceFlow::new("test_client_id");
        assert!(flow.is_ok());
    }
}
