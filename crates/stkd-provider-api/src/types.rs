//! Provider-agnostic types for Git hosting operations.
//!
//! This module defines common data structures that represent concepts
//! across all Git hosting providers (GitHub PRs, GitLab MRs, etc.).
//!
//! # Terminology
//!
//! We use "Merge Request" (MR) as the canonical term, which covers:
//! - GitHub Pull Requests
//! - GitLab Merge Requests
//! - Gitea Pull Requests
//! - Bitbucket Pull Requests

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a merge request.
///
/// This wraps the provider-specific ID (PR number, MR IID, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MergeRequestId(pub u64);

impl From<u64> for MergeRequestId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl fmt::Display for MergeRequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// Repository identifier (owner/name pair).
///
/// # Examples
///
/// ```rust
/// use stkd_provider_api::RepoId;
///
/// let repo = RepoId::new("octocat", "hello-world");
/// assert_eq!(repo.full_name(), "octocat/hello-world");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepoId {
    /// Repository owner (user or organization)
    pub owner: String,
    /// Repository name
    pub name: String,
}

impl RepoId {
    /// Create a new repository identifier.
    pub fn new(owner: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            name: name.into(),
        }
    }

    /// Get the full repository name (owner/name).
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

impl fmt::Display for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

/// State of a merge request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MergeRequestState {
    /// Open and ready for review
    Open,
    /// Closed without merging
    Closed,
    /// Successfully merged
    Merged,
    /// Open but marked as draft/WIP
    Draft,
}

impl MergeRequestState {
    /// Returns `true` if the MR is open (including drafts).
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open | Self::Draft)
    }

    /// Returns `true` if the MR has been merged.
    pub fn is_merged(&self) -> bool {
        matches!(self, Self::Merged)
    }
}

impl fmt::Display for MergeRequestState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Closed => write!(f, "closed"),
            Self::Merged => write!(f, "merged"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

/// A merge request (PR/MR) on a Git hosting platform.
///
/// This is the provider-agnostic representation of a pull request or merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    /// Unique identifier (PR number, MR IID)
    pub id: MergeRequestId,
    /// Display number (usually same as id.0)
    pub number: u64,
    /// Title of the merge request
    pub title: String,
    /// Description/body of the merge request
    pub body: Option<String>,
    /// Current state
    pub state: MergeRequestState,
    /// Source branch name
    pub source_branch: String,
    /// Target branch name
    pub target_branch: String,
    /// Web URL to view the merge request
    pub web_url: String,
    /// Whether this is a draft/WIP
    pub is_draft: bool,
    /// Whether the MR can be merged (no conflicts)
    pub mergeable: Option<bool>,
    /// When the MR was created
    pub created_at: DateTime<Utc>,
    /// When the MR was last updated
    pub updated_at: DateTime<Utc>,
    /// Author username
    pub author: Option<String>,
    /// Labels applied to the MR
    pub labels: Vec<String>,
    /// Assigned milestone
    pub milestone: Option<String>,
}

/// Request to create a new merge request.
#[derive(Debug, Clone, Default)]
pub struct CreateMergeRequest {
    /// Title of the merge request
    pub title: String,
    /// Source branch name
    pub source_branch: String,
    /// Target branch name
    pub target_branch: String,
    /// Description/body
    pub body: Option<String>,
    /// Create as draft/WIP
    pub draft: bool,
    /// Labels to apply
    pub labels: Vec<String>,
    /// Users to assign
    pub assignees: Vec<String>,
    /// Users to request review from
    pub reviewers: Vec<String>,
}

/// Request to update an existing merge request.
#[derive(Debug, Clone, Default)]
pub struct UpdateMergeRequest {
    /// New title (None = keep current)
    pub title: Option<String>,
    /// New description (None = keep current)
    pub body: Option<String>,
    /// New target branch (None = keep current)
    pub target_branch: Option<String>,
    /// New state (None = keep current)
    pub state: Option<MergeRequestState>,
    /// New labels (None = keep current)
    pub labels: Option<Vec<String>>,
    /// New assignees (None = keep current)
    pub assignees: Option<Vec<String>>,
}

/// Method to use when merging a merge request.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MergeMethod {
    /// Standard merge commit
    #[default]
    Merge,
    /// Squash all commits into one
    Squash,
    /// Rebase commits onto target branch
    Rebase,
    /// Fast-forward merge (GitLab only)
    FastForward,
}

impl fmt::Display for MergeMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Merge => write!(f, "merge"),
            Self::Squash => write!(f, "squash"),
            Self::Rebase => write!(f, "rebase"),
            Self::FastForward => write!(f, "fast-forward"),
        }
    }
}

/// Result of a merge operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    /// Whether the merge was successful
    pub merged: bool,
    /// SHA of the merge commit (if successful)
    pub sha: Option<String>,
    /// Message from the merge operation
    pub message: String,
}

/// Status of a CI/CD pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PipelineStatus {
    /// Pipeline is waiting to run
    Pending,
    /// Pipeline is currently running
    Running,
    /// Pipeline completed successfully
    Success,
    /// Pipeline failed
    Failed,
    /// Pipeline was cancelled
    Canceled,
    /// Pipeline was skipped
    Skipped,
    /// Status is unknown or unsupported
    Unknown,
}

impl PipelineStatus {
    /// Returns `true` if the pipeline is in a terminal state.
    pub fn is_finished(&self) -> bool {
        matches!(
            self,
            Self::Success | Self::Failed | Self::Canceled | Self::Skipped
        )
    }

    /// Returns `true` if the pipeline succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }
}

impl fmt::Display for PipelineStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Running => write!(f, "running"),
            Self::Success => write!(f, "success"),
            Self::Failed => write!(f, "failed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Skipped => write!(f, "skipped"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// A CI/CD pipeline run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    /// Pipeline ID
    pub id: u64,
    /// Current status
    pub status: PipelineStatus,
    /// Web URL to view the pipeline
    pub web_url: Option<String>,
    /// Branch or ref that triggered the pipeline
    pub ref_name: String,
    /// Commit SHA
    pub sha: String,
    /// When the pipeline was created
    pub created_at: DateTime<Utc>,
    /// Individual jobs/checks in the pipeline
    pub jobs: Vec<PipelineJob>,
}

/// A single job within a pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineJob {
    /// Job ID
    pub id: u64,
    /// Job name
    pub name: String,
    /// Job status
    pub status: PipelineStatus,
    /// Web URL to view the job
    pub web_url: Option<String>,
}

/// State of a review/approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalState {
    /// Review is pending
    Pending,
    /// Approved
    Approved,
    /// Changes requested
    ChangesRequested,
    /// Reviewer left a comment (no approval decision)
    Commented,
    /// Previous approval was dismissed
    Dismissed,
}

impl ApprovalState {
    /// Returns `true` if this represents an approval.
    pub fn is_approved(&self) -> bool {
        matches!(self, Self::Approved)
    }
}

/// A review on a merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// Review ID
    pub id: u64,
    /// Reviewer username
    pub user: String,
    /// Review state
    pub state: ApprovalState,
    /// Review body/comment
    pub body: Option<String>,
    /// When the review was submitted
    pub submitted_at: Option<DateTime<Utc>>,
}

/// Label on a repository or merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    /// Label name
    pub name: String,
    /// Color (hex code without #)
    pub color: Option<String>,
    /// Description
    pub description: Option<String>,
}

/// State of a milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MilestoneState {
    /// Milestone is open
    Open,
    /// Milestone is closed
    Closed,
}

/// A milestone for tracking progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    /// Milestone ID
    pub id: u64,
    /// Milestone title
    pub title: String,
    /// Description
    pub description: Option<String>,
    /// Due date
    pub due_date: Option<NaiveDate>,
    /// Current state
    pub state: MilestoneState,
}

/// A user on the Git hosting platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: u64,
    /// Username/login
    pub username: String,
    /// Display name
    pub name: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Avatar URL
    pub avatar_url: Option<String>,
}

/// Filter options for listing merge requests.
#[derive(Debug, Clone, Default)]
pub struct MergeRequestFilter {
    /// Filter by state
    pub state: Option<MergeRequestState>,
    /// Filter by source branch
    pub source_branch: Option<String>,
    /// Filter by target branch
    pub target_branch: Option<String>,
    /// Filter by author
    pub author: Option<String>,
    /// Filter by labels
    pub labels: Vec<String>,
    /// Maximum number of results
    pub limit: Option<usize>,
}

/// Branch protection rules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BranchProtection {
    /// Name or pattern of the protected branch
    pub pattern: String,
    /// Whether the branch is protected
    pub is_protected: bool,
    /// Requires pull request before merging
    pub require_pull_request: bool,
    /// Requires approving reviews
    pub required_approvals: u32,
    /// Requires status checks to pass
    pub require_status_checks: bool,
    /// Requires linear history (no merge commits)
    pub require_linear_history: bool,
    /// Allows force pushes
    pub allow_force_push: bool,
    /// Allows deletions
    pub allow_deletions: bool,
}

impl BranchProtection {
    /// Check if this protection prevents force pushes
    pub fn prevents_force_push(&self) -> bool {
        self.is_protected && !self.allow_force_push
    }

    /// Check if this protection prevents deletions
    pub fn prevents_deletion(&self) -> bool {
        self.is_protected && !self.allow_deletions
    }
}

/// Capabilities supported by a provider.
///
/// Providers set these flags to indicate which features they support.
#[derive(Debug, Clone, Default)]
pub struct ProviderCapabilities {
    /// Supports merge request operations
    pub merge_requests: bool,
    /// Supports pipeline/CI status
    pub pipelines: bool,
    /// Supports reviews and approvals
    pub approvals: bool,
    /// Supports labels
    pub labels: bool,
    /// Supports milestones
    pub milestones: bool,
    /// Supports draft/WIP merge requests
    pub draft_mrs: bool,
    /// Supports squash merge
    pub squash_merge: bool,
    /// Supports rebase merge
    pub rebase_merge: bool,
    /// Supports fast-forward merge (GitLab only)
    pub fast_forward_merge: bool,
    /// Supports branch protection queries
    pub branch_protection: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_id() {
        let repo = RepoId::new("owner", "repo");
        assert_eq!(repo.full_name(), "owner/repo");
        assert_eq!(repo.to_string(), "owner/repo");
    }

    #[test]
    fn test_merge_request_id() {
        let id = MergeRequestId::from(42);
        assert_eq!(id.0, 42);
        assert_eq!(id.to_string(), "#42");
    }

    #[test]
    fn test_merge_request_state() {
        assert!(MergeRequestState::Open.is_open());
        assert!(MergeRequestState::Draft.is_open());
        assert!(!MergeRequestState::Closed.is_open());
        assert!(MergeRequestState::Merged.is_merged());
    }

    #[test]
    fn test_pipeline_status() {
        assert!(PipelineStatus::Success.is_finished());
        assert!(PipelineStatus::Failed.is_finished());
        assert!(!PipelineStatus::Running.is_finished());
        assert!(PipelineStatus::Success.is_success());
    }
}
