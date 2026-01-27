//! Pull request operations

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::GitHubClient;

/// Pull request data from GitHub
#[derive(Debug, Clone, Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub html_url: String,
    pub title: String,
    pub body: Option<String>,
    pub state: PrState,
    pub head: PrRef,
    pub base: PrRef,
    pub draft: bool,
    pub mergeable: Option<bool>,
    pub mergeable_state: Option<String>,
}

/// PR state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrState {
    Open,
    Closed,
}

/// Branch reference in a PR
#[derive(Debug, Clone, Deserialize)]
pub struct PrRef {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
}

/// Request to create a pull request
#[derive(Debug, Clone, Serialize)]
pub struct PullRequestCreate {
    pub title: String,
    pub head: String,
    pub base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
}

/// Request to update a pull request
#[derive(Debug, Clone, Serialize)]
pub struct PullRequestUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl GitHubClient {
    /// Create a pull request
    pub async fn create_pr(
        &self,
        owner: &str,
        repo: &str,
        create: &PullRequestCreate,
    ) -> Result<PullRequest> {
        let path = format!("/repos/{}/{}/pulls", owner, repo);
        self.post(&path, create).await
    }

    /// Update a pull request
    pub async fn update_pr(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        update: &PullRequestUpdate,
    ) -> Result<PullRequest> {
        let path = format!("/repos/{}/{}/pulls/{}", owner, repo, number);
        self.patch(&path, update).await
    }

    /// Get a pull request
    pub async fn get_pr(&self, owner: &str, repo: &str, number: u64) -> Result<PullRequest> {
        let path = format!("/repos/{}/{}/pulls/{}", owner, repo, number);
        self.get(&path).await
    }

    /// List pull requests for a branch
    pub async fn list_prs_for_branch(
        &self,
        owner: &str,
        repo: &str,
        head: &str,
    ) -> Result<Vec<PullRequest>> {
        let path = format!(
            "/repos/{}/{}/pulls?head={}:{}&state=open",
            owner, repo, owner, head
        );
        self.get(&path).await
    }

    /// Merge a pull request
    pub async fn merge_pr(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
        merge_method: MergeMethod,
    ) -> Result<MergeResult> {
        let path = format!("/repos/{}/{}/pulls/{}/merge", owner, repo, number);
        let body = serde_json::json!({
            "merge_method": merge_method.as_str()
        });
        self.post(&path, &body).await
    }

    /// Close a pull request
    pub async fn close_pr(&self, owner: &str, repo: &str, number: u64) -> Result<PullRequest> {
        self.update_pr(owner, repo, number, &PullRequestUpdate {
            state: Some("closed".to_string()),
            ..Default::default()
        }).await
    }
}

/// Merge method
#[derive(Debug, Clone, Copy)]
pub enum MergeMethod {
    Merge,
    Squash,
    Rebase,
}

impl MergeMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            MergeMethod::Merge => "merge",
            MergeMethod::Squash => "squash",
            MergeMethod::Rebase => "rebase",
        }
    }
}

/// Merge result
#[derive(Debug, Clone, Deserialize)]
pub struct MergeResult {
    pub sha: String,
    pub merged: bool,
    pub message: String,
}

impl Default for PullRequestUpdate {
    fn default() -> Self {
        Self {
            title: None,
            body: None,
            base: None,
            state: None,
        }
    }
}

/// Generate stack visualization for PR body
pub fn generate_stack_body(
    branches: &[(String, Option<u64>)], // (name, pr_number)
    current_branch: &str,
    original_body: Option<&str>,
) -> String {
    let mut body = String::new();

    // Add stack visualization
    body.push_str("## Stack\n\n");
    body.push_str("```\n");

    for (name, pr_num) in branches {
        let marker = if name == current_branch { "→" } else { " " };
        let pr_info = pr_num.map(|n| format!(" (#{n})")).unwrap_or_default();
        body.push_str(&format!("{} {}{}\n", marker, name, pr_info));
    }

    body.push_str("```\n\n");
    body.push_str("---\n\n");

    // Add original body if present
    if let Some(orig) = original_body {
        body.push_str(orig);
    }

    body
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_stack_body() {
        let branches = vec![
            ("feature/base".to_string(), Some(1)),
            ("feature/auth".to_string(), Some(2)),
            ("feature/tests".to_string(), None),
        ];

        let body = generate_stack_body(&branches, "feature/auth", Some("Original description"));

        assert!(body.contains("## Stack"));
        assert!(body.contains("→ feature/auth"));
        assert!(body.contains("Original description"));
    }
}
