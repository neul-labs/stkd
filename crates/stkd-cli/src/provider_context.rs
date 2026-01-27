//! Provider context for CLI commands.
//!
//! This module handles automatic provider detection and creation
//! based on git remotes and configuration.

use anyhow::{Context, Result};
use stkd_core::Repository;
use stkd_provider_api::{Provider, RepoId, RepositoryProvider};

/// Detected provider type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    GitHub,
    GitLab,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::GitHub => write!(f, "GitHub"),
            ProviderType::GitLab => write!(f, "GitLab"),
        }
    }
}

/// Context for provider operations in CLI commands.
pub struct ProviderContext {
    /// The repository identifier (owner/name)
    pub repo_id: RepoId,
    /// The detected provider type
    pub provider_type: ProviderType,
    /// The provider instance
    provider: Box<dyn Provider>,
}

impl ProviderContext {
    /// Create a provider context from a repository.
    ///
    /// This auto-detects the provider from git remotes and loads
    /// appropriate credentials.
    pub async fn from_repo(repo: &Repository) -> Result<Self> {
        // Get the origin remote URL
        let remote_url = get_remote_url(repo)?;

        // Detect provider type from URL
        let (provider_type, host) = detect_provider(&remote_url)?;

        // Create provider based on type
        let (provider, repo_id): (Box<dyn Provider>, RepoId) = match provider_type {
            ProviderType::GitHub => {
                let auth_token = stkd_github::auth::load_credentials()?
                    .ok_or_else(|| anyhow::anyhow!("Not authenticated with GitHub. Run 'gt auth --github' first."))?;

                let provider = stkd_github::GitHubProvider::with_auth(auth_token.to_auth())?;

                let repo_id = provider.parse_remote_url(&remote_url)
                    .ok_or_else(|| anyhow::anyhow!("Could not parse GitHub repository from remote URL"))?;

                (Box::new(provider), repo_id)
            }
            ProviderType::GitLab => {
                let auth_token = stkd_gitlab::auth::load_credentials(&host)?
                    .ok_or_else(|| anyhow::anyhow!("Not authenticated with GitLab. Run 'gt auth --gitlab' first."))?;

                let provider = if host == "gitlab.com" {
                    stkd_gitlab::GitLabProvider::new(auth_token.token)?
                } else {
                    stkd_gitlab::GitLabProvider::with_host(auth_token.token, &host)?
                };

                let repo_id = provider.parse_remote_url(&remote_url)
                    .ok_or_else(|| anyhow::anyhow!("Could not parse GitLab repository from remote URL"))?;

                (Box::new(provider), repo_id)
            }
        };

        Ok(Self {
            repo_id,
            provider_type,
            provider,
        })
    }

    /// Get a reference to the provider.
    pub fn provider(&self) -> &dyn Provider {
        self.provider.as_ref()
    }

    /// Get the full repository name (owner/repo).
    pub fn full_name(&self) -> String {
        self.repo_id.full_name()
    }
}

/// Get the remote URL from a repository (prefers 'origin').
fn get_remote_url(repo: &Repository) -> Result<String> {
    let git = repo.git();

    // Try origin first
    if let Ok(remote) = git.find_remote("origin") {
        if let Some(url) = remote.url() {
            return Ok(url.to_string());
        }
    }

    // Fall back to first remote
    let remotes = git.remotes().context("Failed to list remotes")?;
    for remote_name in remotes.iter().flatten() {
        if let Ok(remote) = git.find_remote(remote_name) {
            if let Some(url) = remote.url() {
                return Ok(url.to_string());
            }
        }
    }

    anyhow::bail!("No git remote found. Add a remote with 'git remote add origin <url>'")
}

/// Detect the provider type and host from a remote URL.
fn detect_provider(url: &str) -> Result<(ProviderType, String)> {
    // GitHub patterns
    if url.contains("github.com") {
        return Ok((ProviderType::GitHub, "github.com".to_string()));
    }

    // GitLab patterns
    if url.contains("gitlab.com") {
        return Ok((ProviderType::GitLab, "gitlab.com".to_string()));
    }

    // Try to detect self-hosted GitLab from URL structure
    // GitLab URLs typically have paths like /group/subgroup/project
    // GitHub enterprise URLs also exist but are less common

    // Check for SSH format: git@host:path
    if let Some(rest) = url.strip_prefix("git@") {
        if let Some(colon_pos) = rest.find(':') {
            let host = &rest[..colon_pos];

            // Heuristic: if it contains "gitlab" in the hostname, assume GitLab
            if host.to_lowercase().contains("gitlab") {
                return Ok((ProviderType::GitLab, host.to_string()));
            }

            // Otherwise, we can't determine - default to GitHub for now
            // In the future, we could try API endpoints to detect
            return Ok((ProviderType::GitHub, host.to_string()));
        }
    }

    // Check for HTTPS format
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            if host.to_lowercase().contains("gitlab") {
                return Ok((ProviderType::GitLab, host.to_string()));
            }
            // Default to GitHub for unknown hosts
            return Ok((ProviderType::GitHub, host.to_string()));
        }
    }

    anyhow::bail!(
        "Could not detect provider from remote URL: {}\n\
         Supported providers: GitHub, GitLab",
        url
    )
}

/// Check if credentials exist for any provider.
#[allow(dead_code)]
pub fn has_any_credentials() -> bool {
    stkd_github::auth::load_credentials().ok().flatten().is_some()
        || stkd_gitlab::auth::load_credentials("gitlab.com").ok().flatten().is_some()
}

/// Get the detected provider type for a repository without creating a full context.
pub fn detect_provider_type(repo: &Repository) -> Result<ProviderType> {
    let remote_url = get_remote_url(repo)?;
    let (provider_type, _) = detect_provider(&remote_url)?;
    Ok(provider_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_github() {
        let (pt, host) = detect_provider("https://github.com/owner/repo.git").unwrap();
        assert_eq!(pt, ProviderType::GitHub);
        assert_eq!(host, "github.com");

        let (pt, host) = detect_provider("git@github.com:owner/repo.git").unwrap();
        assert_eq!(pt, ProviderType::GitHub);
        assert_eq!(host, "github.com");
    }

    #[test]
    fn test_detect_gitlab() {
        let (pt, host) = detect_provider("https://gitlab.com/group/project.git").unwrap();
        assert_eq!(pt, ProviderType::GitLab);
        assert_eq!(host, "gitlab.com");

        let (pt, host) = detect_provider("git@gitlab.com:group/project.git").unwrap();
        assert_eq!(pt, ProviderType::GitLab);
        assert_eq!(host, "gitlab.com");
    }

    #[test]
    fn test_detect_self_hosted_gitlab() {
        let (pt, host) = detect_provider("https://gitlab.mycompany.com/group/project.git").unwrap();
        assert_eq!(pt, ProviderType::GitLab);
        assert_eq!(host, "gitlab.mycompany.com");

        let (pt, host) = detect_provider("git@gitlab.mycompany.com:group/project.git").unwrap();
        assert_eq!(pt, ProviderType::GitLab);
        assert_eq!(host, "gitlab.mycompany.com");
    }
}
