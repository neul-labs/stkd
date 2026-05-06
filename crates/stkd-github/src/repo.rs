//! Repository info extraction from git remotes

use anyhow::{Context, Result};
use git2::Repository;

/// GitHub repository info
#[derive(Debug, Clone)]
pub struct RepoInfo {
    /// Repository owner (user or org)
    pub owner: String,
    /// Repository name
    pub repo: String,
    /// Remote name (usually "origin")
    pub remote: String,
}

impl RepoInfo {
    /// Parse from a git URL
    pub fn from_url(url: &str, remote: &str) -> Result<Self> {
        let (owner, repo) = parse_github_url(url).context("Failed to parse GitHub URL")?;

        Ok(Self {
            owner,
            repo,
            remote: remote.to_string(),
        })
    }

    /// Detect from a git repository
    pub fn from_repo(repo: &Repository) -> Result<Self> {
        Self::from_repo_with_remote(repo, "origin")
    }

    /// Detect from a git repository with specific remote
    pub fn from_repo_with_remote(repo: &Repository, remote_name: &str) -> Result<Self> {
        let remote = repo
            .find_remote(remote_name)
            .context(format!("Remote '{}' not found", remote_name))?;

        let url = remote.url().context("Remote URL is not valid UTF-8")?;

        Self::from_url(url, remote_name)
    }

    /// Get the full repository path (owner/repo)
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

/// Parse a GitHub URL into (owner, repo)
fn parse_github_url(url: &str) -> Option<(String, String)> {
    // Handle various GitHub URL formats:
    // - https://github.com/owner/repo.git
    // - https://github.com/owner/repo
    // - git@github.com:owner/repo.git
    // - git@github.com:owner/repo
    // - ssh://git@github.com/owner/repo.git
    // - github.com:owner/repo

    let url = url.trim();

    // SSH format: git@github.com:owner/repo.git
    if url.starts_with("git@github.com:") {
        let path = url.strip_prefix("git@github.com:")?;
        return parse_owner_repo(path);
    }

    // SSH URL format: ssh://git@github.com/owner/repo
    if url.starts_with("ssh://git@github.com/") {
        let path = url.strip_prefix("ssh://git@github.com/")?;
        return parse_owner_repo(path);
    }

    // HTTPS format: https://github.com/owner/repo
    if let Some(path) = url.strip_prefix("https://github.com/") {
        return parse_owner_repo(path);
    }

    // HTTP format: http://github.com/owner/repo
    if let Some(path) = url.strip_prefix("http://github.com/") {
        return parse_owner_repo(path);
    }

    // Short format: github.com:owner/repo
    if url.starts_with("github.com:") {
        let path = url.strip_prefix("github.com:")?;
        return parse_owner_repo(path);
    }

    None
}

/// Parse owner/repo from path, stripping .git suffix
fn parse_owner_repo(path: &str) -> Option<(String, String)> {
    let path = path.strip_suffix(".git").unwrap_or(path);
    let path = path.trim_matches('/');

    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_https_url() {
        assert_eq!(
            parse_github_url("https://github.com/owner/repo.git"),
            Some(("owner".to_string(), "repo".to_string()))
        );
        assert_eq!(
            parse_github_url("https://github.com/owner/repo"),
            Some(("owner".to_string(), "repo".to_string()))
        );
    }

    #[test]
    fn test_parse_ssh_url() {
        assert_eq!(
            parse_github_url("git@github.com:owner/repo.git"),
            Some(("owner".to_string(), "repo".to_string()))
        );
        assert_eq!(
            parse_github_url("git@github.com:owner/repo"),
            Some(("owner".to_string(), "repo".to_string()))
        );
    }

    #[test]
    fn test_parse_ssh_protocol_url() {
        assert_eq!(
            parse_github_url("ssh://git@github.com/owner/repo.git"),
            Some(("owner".to_string(), "repo".to_string()))
        );
    }
}
