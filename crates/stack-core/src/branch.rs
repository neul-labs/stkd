//! Branch tracking and metadata

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

    /// Associated pull request number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr_number: Option<u64>,

    /// Pull request URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pr_url: Option<String>,

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

    /// Review status from GitHub
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<ReviewStatus>,

    /// CI status
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
            pr_number: None,
            pr_url: None,
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

    /// Check if this branch has an associated PR
    pub fn has_pr(&self) -> bool {
        self.pr_number.is_some()
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

    /// Set PR info
    pub fn set_pr(&mut self, number: u64, url: impl Into<String>) {
        self.pr_number = Some(number);
        self.pr_url = Some(url.into());
        self.touch();
    }

    /// Clear PR info
    pub fn clear_pr(&mut self) {
        self.pr_number = None;
        self.pr_url = None;
        self.touch();
    }

    /// Update head commit
    pub fn set_head(&mut self, commit: impl Into<String>) {
        self.head_commit = Some(commit.into());
        self.touch();
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

    pub fn pr_number(&self) -> Option<u64> {
        self.info.pr_number
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
        assert!(!info.has_pr());
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
    fn test_branch_set_pr() {
        let mut info = BranchInfo::new("feature/test", "main");
        assert!(!info.has_pr());

        info.set_pr(42, "https://github.com/owner/repo/pull/42");
        assert!(info.has_pr());
        assert_eq!(info.pr_number, Some(42));
    }
}
