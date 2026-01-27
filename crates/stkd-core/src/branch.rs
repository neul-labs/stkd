//! Branch tracking and metadata
//!
//! This module defines the branch information structures used by Stack
//! to track branch relationships and their associated merge requests.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Complete branch information stored by Stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    /// Branch name
    pub name: String,

    /// Parent branch name (what this branch was created from)
    pub parent: String,

    /// Child branches (branches created on top of this one)
    #[serde(default)]
    pub children: Vec<String>,

    /// Associated merge request ID (PR number, MR IID, etc.)
    /// This is the provider-agnostic identifier for the merge request.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "pr_number"
    )]
    pub merge_request_id: Option<u64>,

    /// Merge request URL
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "pr_url"
    )]
    pub merge_request_url: Option<String>,

    /// Provider that hosts this merge request (github, gitlab, gitea, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Commit at the base of this branch (parent's HEAD when created)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_commit: Option<String>,

    /// Current HEAD commit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_commit: Option<String>,

    /// When this branch was created
    pub created_at: DateTime<Utc>,

    /// When this branch was last modified
    pub updated_at: DateTime<Utc>,

    /// Current status
    #[serde(default)]
    pub status: BranchStatus,

    /// Whether this branch is frozen (no local modifications allowed)
    #[serde(default)]
    pub frozen: bool,

    /// Optional description/notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Review status from the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<ReviewStatus>,

    /// CI/Pipeline status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci_status: Option<CiStatus>,
}

impl BranchInfo {
    /// Create new branch info
    pub fn new(name: impl Into<String>, parent: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            name: name.into(),
            parent: parent.into(),
            children: vec![],
            merge_request_id: None,
            merge_request_url: None,
            provider: None,
            base_commit: None,
            head_commit: None,
            created_at: now,
            updated_at: now,
            status: BranchStatus::Active,
            frozen: false,
            description: None,
            review_status: None,
            ci_status: None,
        }
    }

    /// Check if this branch has an associated merge request (PR/MR)
    pub fn has_merge_request(&self) -> bool {
        self.merge_request_id.is_some()
    }

    /// Check if this branch has an associated PR (alias for has_merge_request)
    #[deprecated(note = "Use has_merge_request() instead")]
    pub fn has_pr(&self) -> bool {
        self.has_merge_request()
    }

    /// Add a child branch
    pub fn add_child(&mut self, child: impl Into<String>) {
        let child = child.into();
        if !self.children.contains(&child) {
            self.children.push(child);
        }
    }

    /// Remove a child branch
    pub fn remove_child(&mut self, child: &str) {
        self.children.retain(|c| c != child);
    }

    /// Mark as updated
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Set merge request info
    ///
    /// # Arguments
    ///
    /// * `id` - The merge request ID (PR number, MR IID)
    /// * `url` - The web URL to the merge request
    /// * `provider` - The provider name (e.g., "github", "gitlab")
    pub fn set_merge_request(
        &mut self,
        id: u64,
        url: impl Into<String>,
        provider: impl Into<String>,
    ) {
        self.merge_request_id = Some(id);
        self.merge_request_url = Some(url.into());
        self.provider = Some(provider.into());
        self.touch();
    }

    /// Set PR info (alias for set_merge_request with GitHub as provider)
    #[deprecated(note = "Use set_merge_request() instead")]
    pub fn set_pr(&mut self, number: u64, url: impl Into<String>) {
        self.set_merge_request(number, url, "github");
    }

    /// Clear merge request info
    pub fn clear_merge_request(&mut self) {
        self.merge_request_id = None;
        self.merge_request_url = None;
        self.provider = None;
        self.touch();
    }

    /// Clear PR info (alias for clear_merge_request)
    #[deprecated(note = "Use clear_merge_request() instead")]
    pub fn clear_pr(&mut self) {
        self.clear_merge_request();
    }

    /// Update head commit
    pub fn set_head(&mut self, commit: impl Into<String>) {
        self.head_commit = Some(commit.into());
        self.touch();
    }

    /// Get the merge request ID (alias for merge_request_id field)
    pub fn mr_id(&self) -> Option<u64> {
        self.merge_request_id
    }

    /// Get the merge request URL (alias for merge_request_url field)
    pub fn mr_url(&self) -> Option<&str> {
        self.merge_request_url.as_deref()
    }
}

/// Branch lifecycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BranchStatus {
    /// Branch is active and can be modified
    #[default]
    Active,
    /// PR is submitted and pending review
    Submitted,
    /// Branch has been merged
    Merged,
    /// Branch was closed without merging
    Closed,
    /// Branch is archived (kept for history)
    Archived,
}

impl std::fmt::Display for BranchStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BranchStatus::Active => write!(f, "active"),
            BranchStatus::Submitted => write!(f, "submitted"),
            BranchStatus::Merged => write!(f, "merged"),
            BranchStatus::Closed => write!(f, "closed"),
            BranchStatus::Archived => write!(f, "archived"),
        }
    }
}

/// PR review status from GitHub
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReviewStatus {
    /// No reviews yet
    Pending,
    /// Has reviews, but changes requested
    ChangesRequested,
    /// Approved by reviewers
    Approved,
    /// Review dismissed
    Dismissed,
}

impl std::fmt::Display for ReviewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewStatus::Pending => write!(f, "pending"),
            ReviewStatus::ChangesRequested => write!(f, "changes requested"),
            ReviewStatus::Approved => write!(f, "approved"),
            ReviewStatus::Dismissed => write!(f, "dismissed"),
        }
    }
}

/// CI status from GitHub
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CiStatus {
    /// CI checks pending
    Pending,
    /// CI checks running
    Running,
    /// All CI checks passed
    Passed,
    /// Some CI checks failed
    Failed,
    /// CI checks skipped
    Skipped,
}

impl std::fmt::Display for CiStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CiStatus::Pending => write!(f, "pending"),
            CiStatus::Running => write!(f, "running"),
            CiStatus::Passed => write!(f, "passed"),
            CiStatus::Failed => write!(f, "failed"),
            CiStatus::Skipped => write!(f, "skipped"),
        }
    }
}

/// Lightweight branch reference (for display/iteration)
#[derive(Debug, Clone)]
pub struct Branch<'a> {
    info: &'a BranchInfo,
    is_current: bool,
    depth: usize,
}

impl<'a> Branch<'a> {
    pub fn new(info: &'a BranchInfo, is_current: bool, depth: usize) -> Self {
        Self {
            info,
            is_current,
            depth,
        }
    }

    pub fn name(&self) -> &str {
        &self.info.name
    }

    pub fn parent(&self) -> &str {
        &self.info.parent
    }

    pub fn is_current(&self) -> bool {
        self.is_current
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn info(&self) -> &BranchInfo {
        self.info
    }

    /// Get the merge request ID (PR number, MR IID)
    pub fn merge_request_id(&self) -> Option<u64> {
        self.info.merge_request_id
    }

    /// Alias for merge_request_id for backward compatibility
    #[deprecated(note = "Use merge_request_id() instead")]
    pub fn pr_number(&self) -> Option<u64> {
        self.merge_request_id()
    }

    pub fn status(&self) -> BranchStatus {
        self.info.status
    }

    pub fn review_status(&self) -> Option<ReviewStatus> {
        self.info.review_status
    }

    pub fn ci_status(&self) -> Option<CiStatus> {
        self.info.ci_status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_info_new() {
        let info = BranchInfo::new("feature/test", "main");
        assert_eq!(info.name, "feature/test");
        assert_eq!(info.parent, "main");
        assert!(info.children.is_empty());
        assert!(!info.has_merge_request());
    }

    #[test]
    fn test_branch_add_remove_child() {
        let mut info = BranchInfo::new("feature/base", "main");
        info.add_child("feature/child1");
        info.add_child("feature/child2");
        assert_eq!(info.children.len(), 2);

        info.remove_child("feature/child1");
        assert_eq!(info.children.len(), 1);
        assert_eq!(info.children[0], "feature/child2");
    }

    #[test]
    fn test_branch_set_merge_request() {
        let mut info = BranchInfo::new("feature/test", "main");
        assert!(!info.has_merge_request());

        info.set_merge_request(42, "https://github.com/owner/repo/pull/42", "github");
        assert!(info.has_merge_request());
        assert_eq!(info.merge_request_id, Some(42));
        assert_eq!(info.provider, Some("github".to_string()));
    }

    #[test]
    fn test_branch_clear_merge_request() {
        let mut info = BranchInfo::new("feature/test", "main");
        info.set_merge_request(42, "https://gitlab.com/group/repo/-/merge_requests/42", "gitlab");
        assert!(info.has_merge_request());

        info.clear_merge_request();
        assert!(!info.has_merge_request());
        assert!(info.provider.is_none());
    }

    #[test]
    fn test_branch_mr_accessors() {
        let mut info = BranchInfo::new("feature/test", "main");
        info.set_merge_request(123, "https://example.com/mr/123", "gitlab");

        assert_eq!(info.mr_id(), Some(123));
        assert_eq!(info.mr_url(), Some("https://example.com/mr/123"));
    }

    #[test]
    fn test_backward_compat_serde() {
        // Test that old JSON format with pr_number and pr_url is still readable
        let json = r#"{
            "name": "feature/old",
            "parent": "main",
            "children": [],
            "pr_number": 99,
            "pr_url": "https://github.com/old/repo/pull/99",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "status": "submitted"
        }"#;

        let info: BranchInfo = serde_json::from_str(json).expect("Should parse old format");
        assert_eq!(info.merge_request_id, Some(99));
        assert_eq!(
            info.merge_request_url,
            Some("https://github.com/old/repo/pull/99".to_string())
        );
    }
}
