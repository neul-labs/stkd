//! Init operation - initialize Stack in a repository

use anyhow::{Context, Result};
use serde::Serialize;
use stkd_core::config::{ProviderConfig, StackConfig, SubmitConfig, SyncConfig};
use stkd_core::Repository;

/// Options for the init operation.
#[derive(Debug, Default, Clone)]
pub struct InitOptions {
    /// Trunk branch name (auto-detected if not specified)
    pub trunk: Option<String>,
    /// Remote name (default: origin)
    pub remote: Option<String>,
    /// Create PRs/MRs as draft by default
    pub draft_default: bool,
    /// Delete local branches after merge
    pub delete_merged: bool,
}

/// Result of an init operation.
#[derive(Debug, Serialize)]
pub struct InitResult {
    pub trunk: String,
    pub remote: String,
    pub provider: Option<ProviderConfig>,
    pub draft_default: bool,
    pub delete_merged: bool,
}

/// Detect trunk branch name.
pub fn detect_trunk(repo: &git2::Repository) -> String {
    for candidate in &["main", "master", "develop", "trunk"] {
        if repo.find_branch(candidate, git2::BranchType::Local).is_ok() {
            return candidate.to_string();
        }
    }
    "main".to_string()
}

/// Detect remote name.
pub fn detect_remote(repo: &git2::Repository) -> String {
    if let Ok(remotes) = repo.remotes() {
        if remotes.iter().any(|r| r == Some("origin")) {
            return "origin".to_string();
        }
        if let Some(Some(first)) = remotes.iter().next() {
            return first.to_string();
        }
    }
    "origin".to_string()
}

/// Initialize Stack in a repository.
pub fn init(path: &str, opts: InitOptions) -> Result<InitResult> {
    let git_repo =
        git2::Repository::open(path).context("Not a git repository. Run 'git init' first.")?;

    let trunk = opts.trunk.unwrap_or_else(|| detect_trunk(&git_repo));
    let remote = opts.remote.unwrap_or_else(|| detect_remote(&git_repo));

    let provider_config = if let Ok(remote_obj) = git_repo.find_remote(&remote) {
        if let Some(url) = remote_obj.url() {
            ProviderConfig::from_remote_url(url)
        } else {
            None
        }
    } else {
        None
    };

    let config = StackConfig {
        trunk: trunk.clone(),
        remote: remote.clone(),
        provider: provider_config.clone(),
        submit: SubmitConfig {
            draft: opts.draft_default,
            ..Default::default()
        },
        sync: SyncConfig {
            delete_merged: opts.delete_merged,
            ..Default::default()
        },
        ..Default::default()
    };

    Repository::init_with_config(path, config)?;

    Ok(InitResult {
        trunk,
        remote,
        provider: provider_config,
        draft_default: opts.draft_default,
        delete_merged: opts.delete_merged,
    })
}
