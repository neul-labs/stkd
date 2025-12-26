//! Remote synchronization

use anyhow::Result;
use stack_core::Repository;
use tracing::info;

use crate::api::GitHubClient;
use crate::pr::PrState;

/// Remote sync operations
pub struct RemoteSync<'a> {
    repo: &'a Repository,
    client: GitHubClient,
    owner: String,
    repo_name: String,
}

impl<'a> RemoteSync<'a> {
    /// Create a new sync instance
    pub fn new(
        repo: &'a Repository,
        client: GitHubClient,
        owner: String,
        repo_name: String,
    ) -> Self {
        Self {
            repo,
            client,
            owner,
            repo_name,
        }
    }

    /// Sync branch state with GitHub
    pub async fn sync_branch(&self, branch_name: &str) -> Result<SyncResult> {
        let mut result = SyncResult::default();

        // Get local branch info
        let local_info = self.repo.storage().load_branch(branch_name)?;

        if local_info.is_none() {
            return Ok(result);
        }

        let local_info = local_info.unwrap();

        // Check for PRs
        let prs = self
            .client
            .list_prs_for_branch(&self.owner, &self.repo_name, branch_name)
            .await?;

        if let Some(pr) = prs.first() {
            // Update local info with PR data
            self.repo.storage().update_branch(branch_name, |info| {
                info.merge_request_id = Some(pr.number);
                info.merge_request_url = Some(pr.html_url.clone());
                info.provider = Some("github".to_string());

                match pr.state {
                    PrState::Open => {
                        info.status = stack_core::BranchStatus::Submitted;
                    }
                    PrState::Closed => {
                        if pr.mergeable == Some(true) {
                            info.status = stack_core::BranchStatus::Merged;
                            result.merged_branches.push(branch_name.to_string());
                        } else {
                            info.status = stack_core::BranchStatus::Closed;
                        }
                    }
                }
            })?;

            result.synced_branches.push(branch_name.to_string());
        } else if local_info.merge_request_id.is_some() {
            // PR was closed/merged, clear it
            self.repo.storage().update_branch(branch_name, |info| {
                info.merge_request_id = None;
                info.merge_request_url = None;
                info.provider = None;
            })?;
        }

        Ok(result)
    }

    /// Sync all tracked branches
    pub async fn sync_all(&self) -> Result<SyncResult> {
        let mut result = SyncResult::default();

        let branches = self.repo.storage().list_branches()?;

        for branch in branches {
            let branch_result = self.sync_branch(&branch.name).await?;
            result.merge(branch_result);
        }

        Ok(result)
    }

    /// Push a branch to remote
    pub async fn push_branch(&self, branch_name: &str, force: bool) -> Result<()> {
        let git = self.repo.git();
        let remote_name = &self.repo.config().remote;

        // Find the remote
        let mut remote = git.find_remote(remote_name)?;

        // Set up push options
        let mut push_options = git2::PushOptions::new();
        let mut callbacks = git2::RemoteCallbacks::new();

        // Use credential helper
        callbacks.credentials(|_url, username, _allowed| {
            git2::Cred::ssh_key_from_agent(username.unwrap_or("git"))
        });

        push_options.remote_callbacks(callbacks);

        // Push refspec
        let refspec = if force {
            format!("+refs/heads/{}:refs/heads/{}", branch_name, branch_name)
        } else {
            format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name)
        };

        remote.push(&[&refspec], Some(&mut push_options))?;

        info!("Pushed {} to {}", branch_name, remote_name);

        Ok(())
    }

    /// Fetch updates from remote
    pub async fn fetch(&self) -> Result<()> {
        let git = self.repo.git();
        let remote_name = &self.repo.config().remote;

        let mut remote = git.find_remote(remote_name)?;

        let mut fetch_options = git2::FetchOptions::new();
        let mut callbacks = git2::RemoteCallbacks::new();

        callbacks.credentials(|_url, username, _allowed| {
            git2::Cred::ssh_key_from_agent(username.unwrap_or("git"))
        });

        fetch_options.remote_callbacks(callbacks);

        remote.fetch(&[self.repo.trunk()], Some(&mut fetch_options), None)?;

        info!("Fetched from {}", remote_name);

        Ok(())
    }

    /// Get branches that have been merged on GitHub
    pub async fn get_merged_branches(&self) -> Result<Vec<String>> {
        let mut merged = vec![];

        let branches = self.repo.storage().list_branches()?;

        for branch in branches {
            if let Some(mr_id) = branch.merge_request_id {
                let pr = self
                    .client
                    .get_pr(&self.owner, &self.repo_name, mr_id)
                    .await?;

                if pr.state == PrState::Closed && pr.mergeable == Some(true) {
                    merged.push(branch.name);
                }
            }
        }

        Ok(merged)
    }
}

/// Result of a sync operation
#[derive(Debug, Default)]
pub struct SyncResult {
    /// Branches that were synced
    pub synced_branches: Vec<String>,
    /// Branches that were merged
    pub merged_branches: Vec<String>,
    /// Branches with conflicts
    pub conflict_branches: Vec<String>,
    /// Errors encountered
    pub errors: Vec<String>,
}

impl SyncResult {
    /// Merge another result into this one
    pub fn merge(&mut self, other: SyncResult) {
        self.synced_branches.extend(other.synced_branches);
        self.merged_branches.extend(other.merged_branches);
        self.conflict_branches.extend(other.conflict_branches);
        self.errors.extend(other.errors);
    }

    /// Check if sync was successful
    pub fn is_success(&self) -> bool {
        self.errors.is_empty() && self.conflict_branches.is_empty()
    }
}
