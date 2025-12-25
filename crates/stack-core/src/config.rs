//! Configuration for Stack

use serde::{Deserialize, Serialize};

/// Version of the config schema
pub const CONFIG_VERSION: u32 = 1;

/// Main Stack configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackConfig {
    /// Schema version
    #[serde(default = "default_version")]
    pub version: u32,

    /// Name of the trunk branch (e.g., "main", "master")
    #[serde(default = "default_trunk")]
    pub trunk: String,

    /// Name of the remote (e.g., "origin")
    #[serde(default = "default_remote")]
    pub remote: String,

    /// GitHub configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<GitHubConfig>,

    /// Submit/PR configuration
    #[serde(default)]
    pub submit: SubmitConfig,

    /// Sync configuration
    #[serde(default)]
    pub sync: SyncConfig,

    /// VCS integration settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs: Option<VcsIntegration>,
}

impl Default for StackConfig {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            trunk: "main".to_string(),
            remote: "origin".to_string(),
            github: None,
            submit: SubmitConfig::default(),
            sync: SyncConfig::default(),
            vcs: None,
        }
    }
}

impl StackConfig {
    /// Create a new config with detected values
    pub fn detect(repo: &git2::Repository) -> Self {
        let mut config = Self::default();

        // Try to detect trunk branch
        for candidate in &["main", "master", "develop", "trunk"] {
            if repo.find_branch(candidate, git2::BranchType::Local).is_ok() {
                config.trunk = candidate.to_string();
                break;
            }
        }

        // Try to detect remote
        if let Ok(remotes) = repo.remotes() {
            if remotes.iter().any(|r| r == Some("origin")) {
                config.remote = "origin".to_string();
            } else if let Some(Some(first)) = remotes.iter().next() {
                config.remote = first.to_string();
            }
        }

        // Try to detect GitHub info from remote URL
        if let Ok(remote) = repo.find_remote(&config.remote) {
            if let Some(url) = remote.url() {
                config.github = GitHubConfig::from_remote_url(url);
            }
        }

        config
    }
}

/// GitHub-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    /// Repository owner (user or organization)
    pub owner: String,

    /// Repository name
    pub repo: String,

    /// GitHub API base URL (for enterprise)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
}

impl GitHubConfig {
    /// Try to parse GitHub info from a remote URL
    pub fn from_remote_url(url: &str) -> Option<Self> {
        // Handle SSH URLs: git@github.com:owner/repo.git
        if url.starts_with("git@github.com:") {
            let path = url.strip_prefix("git@github.com:")?;
            let path = path.strip_suffix(".git").unwrap_or(path);
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() == 2 {
                return Some(Self {
                    owner: parts[0].to_string(),
                    repo: parts[1].to_string(),
                    api_url: None,
                });
            }
        }

        // Handle HTTPS URLs: https://github.com/owner/repo.git
        if url.contains("github.com") {
            let url = url.strip_prefix("https://").or_else(|| url.strip_prefix("http://"))?;
            let url = url.strip_prefix("github.com/")?;
            let url = url.strip_suffix(".git").unwrap_or(url);
            let parts: Vec<&str> = url.split('/').collect();
            if parts.len() >= 2 {
                return Some(Self {
                    owner: parts[0].to_string(),
                    repo: parts[1].to_string(),
                    api_url: None,
                });
            }
        }

        None
    }
}

/// Configuration for PR submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitConfig {
    /// Create PRs as draft by default
    #[serde(default)]
    pub draft: bool,

    /// Automatically generate title from branch name
    #[serde(default = "default_true")]
    pub auto_title: bool,

    /// Use PR template if available
    #[serde(default = "default_true")]
    pub pr_template: bool,

    /// Add stack visualization to PR body
    #[serde(default = "default_true")]
    pub include_stack_info: bool,

    /// Push without creating PRs
    #[serde(default)]
    pub push_only: bool,
}

impl Default for SubmitConfig {
    fn default() -> Self {
        Self {
            draft: false,
            auto_title: true,
            pr_template: true,
            include_stack_info: true,
            push_only: false,
        }
    }
}

/// Configuration for sync behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Delete branches after they're merged
    #[serde(default = "default_true")]
    pub delete_merged: bool,

    /// Prompt before deleting branches
    #[serde(default = "default_true")]
    pub prompt_delete: bool,

    /// Automatically restack after sync
    #[serde(default = "default_true")]
    pub auto_restack: bool,

    /// Pull trunk before operations
    #[serde(default = "default_true")]
    pub pull_trunk: bool,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            delete_merged: true,
            prompt_delete: true,
            auto_restack: true,
            pull_trunk: true,
        }
    }
}

/// VCS integration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcsIntegration {
    /// Enable VCS integration
    #[serde(default)]
    pub enabled: bool,

    /// Share intent metadata with VCS
    #[serde(default)]
    pub share_intent: bool,

    /// Respect VCS policy gates before landing
    #[serde(default)]
    pub respect_policy_gates: bool,
}

impl Default for VcsIntegration {
    fn default() -> Self {
        Self {
            enabled: false,
            share_intent: false,
            respect_policy_gates: false,
        }
    }
}

// Default value helpers
fn default_version() -> u32 {
    CONFIG_VERSION
}

fn default_trunk() -> String {
    "main".to_string()
}

fn default_remote() -> String {
    "origin".to_string()
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_config_from_ssh_url() {
        let config = GitHubConfig::from_remote_url("git@github.com:owner/repo.git");
        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.owner, "owner");
        assert_eq!(config.repo, "repo");
    }

    #[test]
    fn test_github_config_from_https_url() {
        let config = GitHubConfig::from_remote_url("https://github.com/owner/repo.git");
        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.owner, "owner");
        assert_eq!(config.repo, "repo");
    }

    #[test]
    fn test_github_config_from_https_url_no_git_suffix() {
        let config = GitHubConfig::from_remote_url("https://github.com/owner/repo");
        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.owner, "owner");
        assert_eq!(config.repo, "repo");
    }

    #[test]
    fn test_default_config() {
        let config = StackConfig::default();
        assert_eq!(config.trunk, "main");
        assert_eq!(config.remote, "origin");
        assert!(!config.submit.draft);
        assert!(config.submit.auto_title);
    }
}
