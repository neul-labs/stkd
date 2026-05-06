//! Configuration for Stack
//!
//! This module defines the configuration structures for Stack, including
//! provider configuration for GitHub, GitLab, and other Git hosting platforms.
//!
//! # Configuration File
//!
//! Stack stores its configuration in `.git/stkd/config.json`. The configuration
//! is automatically created when running `gt init` and includes:
//!
//! - Trunk branch name (e.g., "main")
//! - Remote name (e.g., "origin")
//! - Provider settings (GitHub, GitLab, etc.)
//! - Submit and sync preferences
//!
//! # Version Migration
//!
//! The configuration format has evolved over time. When loading older configs,
//! Stack automatically migrates them to the current version and saves the result.
//!
//! # Example
//!
//! ```json
//! {
//!   "version": 2,
//!   "trunk": "main",
//!   "remote": "origin",
//!   "provider": {
//!     "type": "github",
//!     "owner": "user",
//!     "repo": "project",
//!     "host": "github.com"
//!   },
//!   "submit": {
//!     "draft": false,
//!     "auto_title": true
//!   }
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Current version of the configuration schema.
///
/// When the schema changes, this version is incremented and migration
/// logic is added to handle older configurations.
pub const CONFIG_VERSION: u32 = 2;

/// Main Stack configuration.
///
/// This struct holds all configuration options for a Stack-enabled repository.
/// It's stored in `.git/stkd/config.json` and created during `gt init`.
///
/// # Fields
///
/// - `trunk`: The main branch that stacks are built on top of
/// - `remote`: The Git remote to push to (usually "origin")
/// - `provider`: Configuration for the Git hosting provider (GitHub, GitLab, etc.)
/// - `submit`: Options for creating merge/pull requests
/// - `sync`: Options for syncing with remote
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

    /// Provider configuration (new unified format)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderConfig>,

    /// GitHub configuration (legacy, for backward compatibility)
    /// This will be migrated to `provider` on first load
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
            provider: None,
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

        // Try to detect provider info from remote URL
        if let Ok(remote) = repo.find_remote(&config.remote) {
            if let Some(url) = remote.url() {
                config.provider = ProviderConfig::from_remote_url(url);
            }
        }

        config
    }

    /// Migrate legacy configuration to the new format.
    ///
    /// This converts the old `github` field to the new `provider` field.
    /// Should be called when loading configs with version < 2.
    pub fn migrate(&mut self) {
        // Migrate legacy github config to provider config
        if self.provider.is_none() {
            if let Some(github) = self.github.take() {
                self.provider = Some(ProviderConfig {
                    provider_type: ProviderType::GitHub,
                    owner: Some(github.owner),
                    repo: Some(github.repo),
                    api_url: github.api_url,
                    web_url: None,
                    host: Some("github.com".to_string()),
                });
                self.version = CONFIG_VERSION;
            }
        }
    }

    /// Get the effective provider configuration.
    ///
    /// This returns the provider config, falling back to converting
    /// the legacy github config if necessary.
    pub fn effective_provider(&self) -> Option<ProviderConfig> {
        if let Some(ref provider) = self.provider {
            Some(provider.clone())
        } else {
            self.github.as_ref().map(|github| ProviderConfig {
                provider_type: ProviderType::GitHub,
                owner: Some(github.owner.clone()),
                repo: Some(github.repo.clone()),
                api_url: github.api_url.clone(),
                web_url: None,
                host: Some("github.com".to_string()),
            })
        }
    }
}

/// GitHub-specific configuration (legacy format).
///
/// This structure is preserved for backward compatibility with v1 configurations.
/// New configurations use [`ProviderConfig`] instead. When a v1 config is loaded,
/// it's automatically migrated to use `ProviderConfig`.
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
            let url = url
                .strip_prefix("https://")
                .or_else(|| url.strip_prefix("http://"))?;
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

/// Supported Git hosting providers.
///
/// Stack supports multiple Git hosting platforms. The provider type determines
/// which API to use for creating merge requests, checking CI status, etc.
///
/// # Auto-Detection
///
/// When set to [`Auto`](Self::Auto), Stack will detect the provider from the
/// remote URL. For example:
/// - `git@github.com:user/repo.git` → GitHub
/// - `https://gitlab.com/group/project.git` → GitLab
/// - `https://codeberg.org/user/repo.git` → Gitea
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    /// Auto-detect provider from the remote URL.
    #[default]
    Auto,
    /// GitHub (github.com or GitHub Enterprise).
    GitHub,
    /// GitLab (gitlab.com or self-hosted instances).
    GitLab,
    /// Gitea or Codeberg (self-hosted Gitea instances).
    Gitea,
}

impl ProviderType {
    /// Detect provider type from a remote URL
    pub fn from_remote_url(url: &str) -> Self {
        let url_lower = url.to_lowercase();

        if url_lower.contains("github.com") || url_lower.contains("github") {
            ProviderType::GitHub
        } else if url_lower.contains("gitlab.com") || url_lower.contains("gitlab") {
            ProviderType::GitLab
        } else if url_lower.contains("gitea") || url_lower.contains("codeberg") {
            ProviderType::Gitea
        } else {
            // Default to GitHub for unknown providers (most common)
            ProviderType::GitHub
        }
    }

    /// Get the default API URL for cloud instances
    pub fn default_api_url(&self) -> Option<&'static str> {
        match self {
            ProviderType::GitHub => Some("https://api.github.com"),
            ProviderType::GitLab => Some("https://gitlab.com/api/v4"),
            ProviderType::Gitea => None, // No default, must be specified
            ProviderType::Auto => None,
        }
    }

    /// Get the default web URL for cloud instances
    pub fn default_web_url(&self) -> Option<&'static str> {
        match self {
            ProviderType::GitHub => Some("https://github.com"),
            ProviderType::GitLab => Some("https://gitlab.com"),
            ProviderType::Gitea => None,
            ProviderType::Auto => None,
        }
    }

    /// Get the default host for cloud instances
    pub fn default_host(&self) -> Option<&'static str> {
        match self {
            ProviderType::GitHub => Some("github.com"),
            ProviderType::GitLab => Some("gitlab.com"),
            ProviderType::Gitea => None,
            ProviderType::Auto => None,
        }
    }

    /// Get the internal name of this provider
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderType::Auto => "auto",
            ProviderType::GitHub => "github",
            ProviderType::GitLab => "gitlab",
            ProviderType::Gitea => "gitea",
        }
    }

    /// Get the display name of this provider
    pub fn display_name(&self) -> &'static str {
        match self {
            ProviderType::Auto => "Auto",
            ProviderType::GitHub => "GitHub",
            ProviderType::GitLab => "GitLab",
            ProviderType::Gitea => "Gitea",
        }
    }
}

impl fmt::Display for ProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Provider configuration (unified format for all providers).
///
/// This structure holds configuration for any supported Git hosting provider.
/// It replaces the legacy `GitHubConfig` and provides a unified interface
/// for GitHub, GitLab, Gitea, and other providers.
///
/// # Self-Hosted Instances
///
/// For self-hosted instances (GitHub Enterprise, self-hosted GitLab, etc.),
/// set the `api_url` and optionally `web_url` fields:
///
/// ```json
/// {
///   "type": "gitlab",
///   "owner": "myteam",
///   "repo": "myproject",
///   "api_url": "https://gitlab.mycompany.com/api/v4",
///   "host": "gitlab.mycompany.com"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    /// Provider type (auto-detected if not specified)
    #[serde(default, rename = "type")]
    pub provider_type: ProviderType,

    /// Repository owner (user or organization)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    /// Repository name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,

    /// API base URL (for self-hosted instances)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,

    /// Web base URL (for self-hosted instances, if different from API)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,

    /// Host identifier (for credential lookup, e.g., "github.com", "gitlab.mycompany.com")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl ProviderConfig {
    /// Create a new provider config from a remote URL
    pub fn from_remote_url(url: &str) -> Option<Self> {
        let (owner, repo, host) = parse_remote_url(url)?;
        let provider_type = ProviderType::from_remote_url(url);

        Some(Self {
            provider_type,
            owner: Some(owner),
            repo: Some(repo),
            api_url: None, // Use defaults for cloud
            web_url: None,
            host: Some(host),
        })
    }

    /// Get the effective API URL
    pub fn effective_api_url(&self) -> Option<String> {
        self.api_url
            .clone()
            .or_else(|| self.provider_type.default_api_url().map(String::from))
    }

    /// Get the effective web URL
    pub fn effective_web_url(&self) -> Option<String> {
        self.web_url
            .clone()
            .or_else(|| self.provider_type.default_web_url().map(String::from))
    }

    /// Get the effective host for credential lookup
    pub fn effective_host(&self) -> String {
        self.host.clone().unwrap_or_else(|| {
            self.provider_type
                .default_host()
                .unwrap_or("unknown")
                .to_string()
        })
    }

    /// Get the full repository name (owner/repo)
    pub fn full_name(&self) -> Option<String> {
        match (&self.owner, &self.repo) {
            (Some(owner), Some(repo)) => Some(format!("{}/{}", owner, repo)),
            _ => None,
        }
    }
}

/// Parse a remote URL into (owner, repo, host)
fn parse_remote_url(url: &str) -> Option<(String, String, String)> {
    // Handle SSH format: git@host:owner/repo.git
    if let Some(rest) = url.strip_prefix("git@") {
        let parts: Vec<&str> = rest.splitn(2, ':').collect();
        if parts.len() == 2 {
            let host = parts[0].to_string();
            let path = parts[1].strip_suffix(".git").unwrap_or(parts[1]);
            let path_parts: Vec<&str> = path.split('/').collect();
            if path_parts.len() >= 2 {
                return Some((path_parts[0].to_string(), path_parts[1].to_string(), host));
            }
        }
    }

    // Handle HTTPS format: https://host/owner/repo.git
    let url_without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;

    let parts: Vec<&str> = url_without_scheme.splitn(2, '/').collect();
    if parts.len() == 2 {
        let host = parts[0].to_string();
        let path = parts[1].strip_suffix(".git").unwrap_or(parts[1]);
        let path_parts: Vec<&str> = path.split('/').collect();
        if path_parts.len() >= 2 {
            return Some((path_parts[0].to_string(), path_parts[1].to_string(), host));
        }
    }

    None
}

/// Configuration for merge/pull request submission.
///
/// These settings control how `gt submit` creates and updates merge requests.
///
/// # Example
///
/// ```json
/// {
///   "submit": {
///     "draft": true,
///     "auto_title": true,
///     "pr_template": true,
///     "include_stack_info": true
///   }
/// }
/// ```
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

/// Configuration for repository synchronization.
///
/// These settings control how `gt sync` behaves when syncing with the remote
/// repository and cleaning up merged branches.
///
/// # Example
///
/// ```json
/// {
///   "sync": {
///     "delete_merged": true,
///     "prompt_delete": false,
///     "auto_restack": true,
///     "pull_trunk": true
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Automatically delete local branches after their MRs are merged.
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

/// VCS integration settings for enterprise environments.
///
/// These settings control integration with Version Control Systems and
/// code review policies. This is primarily useful for enterprise environments
/// with additional policy enforcement requirements.
///
/// **Note**: This feature is currently experimental and may change.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VcsIntegration {
    /// Enable VCS integration features.
    #[serde(default)]
    pub enabled: bool,

    /// Share intent metadata with the VCS system.
    /// When enabled, Stack will communicate branch intent to the VCS.
    #[serde(default)]
    pub share_intent: bool,

    /// Respect VCS policy gates before landing.
    /// When enabled, `gt land` will check policy gates before merging.
    #[serde(default)]
    pub respect_policy_gates: bool,
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

    #[test]
    fn test_provider_type_from_github_url() {
        assert_eq!(
            ProviderType::from_remote_url("git@github.com:owner/repo.git"),
            ProviderType::GitHub
        );
        assert_eq!(
            ProviderType::from_remote_url("https://github.com/owner/repo.git"),
            ProviderType::GitHub
        );
    }

    #[test]
    fn test_provider_type_from_gitlab_url() {
        assert_eq!(
            ProviderType::from_remote_url("git@gitlab.com:owner/repo.git"),
            ProviderType::GitLab
        );
        assert_eq!(
            ProviderType::from_remote_url("https://gitlab.mycompany.com/group/project.git"),
            ProviderType::GitLab
        );
    }

    #[test]
    fn test_provider_type_from_gitea_url() {
        assert_eq!(
            ProviderType::from_remote_url("https://codeberg.org/owner/repo.git"),
            ProviderType::Gitea
        );
    }

    #[test]
    fn test_provider_config_from_github_ssh() {
        let config = ProviderConfig::from_remote_url("git@github.com:owner/repo.git");
        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.provider_type, ProviderType::GitHub);
        assert_eq!(config.owner, Some("owner".to_string()));
        assert_eq!(config.repo, Some("repo".to_string()));
        assert_eq!(config.host, Some("github.com".to_string()));
    }

    #[test]
    fn test_provider_config_from_gitlab_https() {
        let config = ProviderConfig::from_remote_url("https://gitlab.com/group/project.git");
        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.provider_type, ProviderType::GitLab);
        assert_eq!(config.owner, Some("group".to_string()));
        assert_eq!(config.repo, Some("project".to_string()));
        assert_eq!(config.host, Some("gitlab.com".to_string()));
    }

    #[test]
    fn test_provider_config_effective_urls() {
        let config = ProviderConfig {
            provider_type: ProviderType::GitHub,
            owner: Some("owner".to_string()),
            repo: Some("repo".to_string()),
            api_url: None,
            web_url: None,
            host: None,
        };

        assert_eq!(
            config.effective_api_url(),
            Some("https://api.github.com".to_string())
        );
        assert_eq!(
            config.effective_web_url(),
            Some("https://github.com".to_string())
        );
        assert_eq!(config.effective_host(), "github.com");
    }

    #[test]
    fn test_provider_config_custom_urls() {
        let config = ProviderConfig {
            provider_type: ProviderType::GitLab,
            owner: Some("owner".to_string()),
            repo: Some("repo".to_string()),
            api_url: Some("https://gitlab.mycompany.com/api/v4".to_string()),
            web_url: Some("https://gitlab.mycompany.com".to_string()),
            host: Some("gitlab.mycompany.com".to_string()),
        };

        assert_eq!(
            config.effective_api_url(),
            Some("https://gitlab.mycompany.com/api/v4".to_string())
        );
        assert_eq!(config.effective_host(), "gitlab.mycompany.com");
    }

    #[test]
    fn test_config_migration() {
        let mut config = StackConfig {
            version: 1,
            trunk: "main".to_string(),
            remote: "origin".to_string(),
            provider: None,
            github: Some(GitHubConfig {
                owner: "oldowner".to_string(),
                repo: "oldrepo".to_string(),
                api_url: None,
            }),
            submit: SubmitConfig::default(),
            sync: SyncConfig::default(),
            vcs: None,
        };

        config.migrate();

        assert!(config.provider.is_some());
        assert!(config.github.is_none()); // Should be consumed
        assert_eq!(config.version, CONFIG_VERSION);

        let provider = config.provider.unwrap();
        assert_eq!(provider.provider_type, ProviderType::GitHub);
        assert_eq!(provider.owner, Some("oldowner".to_string()));
        assert_eq!(provider.repo, Some("oldrepo".to_string()));
    }

    #[test]
    fn test_effective_provider_with_legacy() {
        let config = StackConfig {
            version: 1,
            trunk: "main".to_string(),
            remote: "origin".to_string(),
            provider: None,
            github: Some(GitHubConfig {
                owner: "legacyowner".to_string(),
                repo: "legacyrepo".to_string(),
                api_url: None,
            }),
            submit: SubmitConfig::default(),
            sync: SyncConfig::default(),
            vcs: None,
        };

        let effective = config.effective_provider();
        assert!(effective.is_some());
        let effective = effective.unwrap();
        assert_eq!(effective.owner, Some("legacyowner".to_string()));
    }

    #[test]
    fn test_provider_type_display() {
        assert_eq!(ProviderType::GitHub.to_string(), "GitHub");
        assert_eq!(ProviderType::GitLab.to_string(), "GitLab");
        assert_eq!(ProviderType::Gitea.to_string(), "Gitea");
    }
}
