//! GitHub provider implementation.
//!
//! This module provides the `GitHubProvider` struct which implements the
//! provider traits from `stack-provider-api`, enabling Stack to work with
//! GitHub repositories.
//!
//! # Example
//!
//! ```rust,ignore
//! use stack_github::GitHubProvider;
//! use stack_provider_api::{Provider, RepoId};
//!
//! let provider = GitHubProvider::new("ghp_token123")?;
//! let repo = RepoId::new("owner", "repo");
//!
//! // Create a merge request
//! let mr = provider.create_mr(&repo, CreateMergeRequest {
//!     title: "My PR".to_string(),
//!     source_branch: "feature".to_string(),
//!     target_branch: "main".to_string(),
//!     ..Default::default()
//! }).await?;
//! ```

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use stack_provider_api::error::{ProviderError, ProviderResult};
use stack_provider_api::traits::*;
use stack_provider_api::types::*;

use crate::auth::GitHubAuth;

/// GitHub provider implementing the Stack provider traits.
///
/// This is the main entry point for GitHub integration in Stack.
/// It provides implementations for merge requests, users, repositories,
/// and optionally pipelines, approvals, and labels.
pub struct GitHubProvider {
    client: reqwest::Client,
    base_url: String,
    auth: GitHubAuth,
}

impl GitHubProvider {
    /// Create a new GitHub provider with a personal access token.
    ///
    /// # Arguments
    ///
    /// * `token` - A GitHub personal access token with appropriate scopes
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn new(token: impl Into<String>) -> ProviderResult<Self> {
        Self::with_auth(GitHubAuth::Token(token.into()))
    }

    /// Create a new GitHub provider with authentication.
    ///
    /// # Arguments
    ///
    /// * `auth` - GitHub authentication (token or OAuth)
    pub fn with_auth(auth: GitHubAuth) -> ProviderResult<Self> {
        Self::with_base_url(auth, "https://api.github.com")
    }

    /// Create a provider for GitHub Enterprise.
    ///
    /// # Arguments
    ///
    /// * `auth` - GitHub authentication
    /// * `base_url` - The base URL of the GitHub Enterprise API
    pub fn with_base_url(auth: GitHubAuth, base_url: &str) -> ProviderResult<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github+json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("stack-cli/0.1.0"));
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| ProviderError::Internal(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            auth,
        })
    }

    /// Make a GET request to the GitHub API.
    async fn get<T: DeserializeOwned>(&self, path: &str) -> ProviderResult<T> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Make a POST request to the GitHub API.
    async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> ProviderResult<T> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .json(body)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Make a PATCH request to the GitHub API.
    async fn patch<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> ProviderResult<T> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .patch(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .json(body)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Make a PUT request to the GitHub API.
    async fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> ProviderResult<T> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .put(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .json(body)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        self.handle_response(response).await
    }

    /// Make a DELETE request to the GitHub API.
    async fn delete(&self, path: &str) -> ProviderResult<()> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth.token()))
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        let status = response.status();
        if status.is_success() || status == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let text = response.text().await.unwrap_or_default();
            Err(self.parse_error_response(status.as_u16(), &text))
        }
    }

    /// Handle API response and parse errors.
    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> ProviderResult<T> {
        let status = response.status();

        if status.is_success() {
            response
                .json()
                .await
                .map_err(|e| ProviderError::Internal(format!("Failed to parse response: {}", e)))
        } else {
            let text = response.text().await.unwrap_or_default();
            Err(self.parse_error_response(status.as_u16(), &text))
        }
    }

    /// Parse an error response from GitHub.
    fn parse_error_response(&self, status: u16, body: &str) -> ProviderError {
        match status {
            401 => ProviderError::AuthenticationFailed("Invalid or expired token".to_string()),
            403 => {
                if body.contains("rate limit") {
                    ProviderError::RateLimited { retry_after: None }
                } else {
                    ProviderError::AuthorizationDenied(body.to_string())
                }
            }
            404 => ProviderError::NotFound(body.to_string()),
            409 => ProviderError::MergeConflict(body.to_string()),
            422 => ProviderError::ValidationError(body.to_string()),
            _ => ProviderError::ProviderSpecific(format!("GitHub API error ({}): {}", status, body)),
        }
    }
}

// GitHub API response types

#[derive(Debug, Deserialize)]
struct GhPullRequest {
    number: u64,
    html_url: String,
    title: String,
    body: Option<String>,
    state: String,
    head: GhRef,
    base: GhRef,
    draft: bool,
    mergeable: Option<bool>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user: Option<GhUser>,
    labels: Vec<GhLabel>,
    milestone: Option<GhMilestone>,
    merged: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct GhRef {
    #[serde(rename = "ref")]
    ref_name: String,
}

#[derive(Debug, Deserialize)]
struct GhUser {
    id: u64,
    login: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GhLabel {
    name: String,
    color: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GhMilestone {
    id: u64,
    #[allow(dead_code)]
    number: u64,
    title: String,
    description: Option<String>,
    state: String,
    due_on: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
struct GhMergeResult {
    sha: Option<String>,
    merged: bool,
    message: String,
}

#[derive(Debug, Deserialize)]
struct GhRepository {
    default_branch: String,
}

#[derive(Debug, Deserialize)]
struct GhReview {
    id: u64,
    user: Option<GhUser>,
    state: String,
    body: Option<String>,
    submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
struct GhCheckRun {
    id: u64,
    name: String,
    status: String,
    conclusion: Option<String>,
    html_url: Option<String>,
}

// GhCheckSuite is kept for future use with GitHub Actions workflow runs
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GhCheckSuite {
    id: u64,
    status: String,
    conclusion: Option<String>,
    head_branch: String,
    head_sha: String,
    created_at: DateTime<Utc>,
    check_runs: Option<Vec<GhCheckRun>>,
}

#[derive(Debug, Deserialize)]
struct GhCheckRunsResponse {
    total_count: u64,
    check_runs: Vec<GhCheckRun>,
}

// Request types

#[derive(Debug, Serialize)]
struct GhCreatePullRequest {
    title: String,
    head: String,
    base: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<bool>,
}

#[derive(Debug, Serialize)]
struct GhUpdatePullRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

#[derive(Debug, Serialize)]
struct GhMergeRequest {
    merge_method: String,
}

#[derive(Debug, Serialize)]
struct GhReviewersRequest {
    reviewers: Vec<String>,
}

#[derive(Debug, Serialize)]
struct GhLabelsRequest {
    labels: Vec<String>,
}

// Conversion helpers

impl From<GhPullRequest> for MergeRequest {
    fn from(pr: GhPullRequest) -> Self {
        let state = if pr.merged == Some(true) {
            MergeRequestState::Merged
        } else if pr.state == "closed" {
            MergeRequestState::Closed
        } else if pr.draft {
            MergeRequestState::Draft
        } else {
            MergeRequestState::Open
        };

        Self {
            id: MergeRequestId(pr.number),
            number: pr.number,
            title: pr.title,
            body: pr.body,
            state,
            source_branch: pr.head.ref_name,
            target_branch: pr.base.ref_name,
            web_url: pr.html_url,
            is_draft: pr.draft,
            mergeable: pr.mergeable,
            created_at: pr.created_at,
            updated_at: pr.updated_at,
            author: pr.user.map(|u| u.login),
            labels: pr.labels.into_iter().map(|l| l.name).collect(),
            milestone: pr.milestone.map(|m| m.title),
        }
    }
}

impl From<GhUser> for User {
    fn from(user: GhUser) -> Self {
        Self {
            id: user.id,
            username: user.login,
            name: user.name,
            email: user.email,
            avatar_url: user.avatar_url,
        }
    }
}

impl From<GhLabel> for Label {
    fn from(label: GhLabel) -> Self {
        Self {
            name: label.name,
            color: label.color,
            description: label.description,
        }
    }
}

impl From<GhReview> for Review {
    fn from(review: GhReview) -> Self {
        let state = match review.state.as_str() {
            "APPROVED" => ApprovalState::Approved,
            "CHANGES_REQUESTED" => ApprovalState::ChangesRequested,
            "DISMISSED" => ApprovalState::Dismissed,
            "COMMENTED" => ApprovalState::Commented,
            _ => ApprovalState::Pending,
        };

        Self {
            id: review.id,
            user: review.user.map(|u| u.login).unwrap_or_default(),
            state,
            body: review.body,
            submitted_at: review.submitted_at,
        }
    }
}

impl From<GhMilestone> for Milestone {
    fn from(m: GhMilestone) -> Self {
        let state = if m.state == "closed" {
            MilestoneState::Closed
        } else {
            MilestoneState::Open
        };

        Self {
            id: m.id,
            title: m.title,
            description: m.description,
            due_date: m.due_on.map(|d| d.date_naive()),
            state,
        }
    }
}

fn check_status_to_pipeline_status(status: &str, conclusion: Option<&str>) -> PipelineStatus {
    match status {
        "queued" | "in_progress" => match conclusion {
            None => PipelineStatus::Running,
            _ => PipelineStatus::Pending,
        },
        "completed" => match conclusion {
            Some("success") => PipelineStatus::Success,
            Some("failure") | Some("timed_out") => PipelineStatus::Failed,
            Some("cancelled") | Some("stale") => PipelineStatus::Canceled,
            Some("skipped") => PipelineStatus::Skipped,
            _ => PipelineStatus::Unknown,
        },
        _ => PipelineStatus::Unknown,
    }
}

// Provider trait implementations

#[async_trait]
impl MergeRequestProvider for GitHubProvider {
    async fn create_mr(
        &self,
        repo: &RepoId,
        request: CreateMergeRequest,
    ) -> ProviderResult<MergeRequest> {
        let path = format!("/repos/{}/{}/pulls", repo.owner, repo.name);

        let body = GhCreatePullRequest {
            title: request.title,
            head: request.source_branch,
            base: request.target_branch,
            body: request.body,
            draft: if request.draft { Some(true) } else { None },
        };

        let pr: GhPullRequest = self.post(&path, &body).await?;
        let mr = MergeRequest::from(pr);

        // Add labels if specified
        if !request.labels.is_empty() {
            let labels_path = format!("/repos/{}/{}/issues/{}/labels", repo.owner, repo.name, mr.number);
            let _ = self.post::<Vec<GhLabel>, _>(&labels_path, &GhLabelsRequest { labels: request.labels }).await;
        }

        // Request reviewers if specified
        if !request.reviewers.is_empty() {
            let reviewers_path = format!("/repos/{}/{}/pulls/{}/requested_reviewers", repo.owner, repo.name, mr.number);
            let _ = self.post::<serde_json::Value, _>(&reviewers_path, &GhReviewersRequest { reviewers: request.reviewers }).await;
        }

        Ok(mr)
    }

    async fn update_mr(
        &self,
        repo: &RepoId,
        id: MergeRequestId,
        update: UpdateMergeRequest,
    ) -> ProviderResult<MergeRequest> {
        let path = format!("/repos/{}/{}/pulls/{}", repo.owner, repo.name, id.0);

        let state = update.state.map(|s| match s {
            MergeRequestState::Open | MergeRequestState::Draft => "open".to_string(),
            MergeRequestState::Closed | MergeRequestState::Merged => "closed".to_string(),
        });

        let body = GhUpdatePullRequest {
            title: update.title,
            body: update.body,
            base: update.target_branch,
            state,
        };

        let pr: GhPullRequest = self.patch(&path, &body).await?;

        // Update labels if specified
        if let Some(labels) = update.labels {
            let labels_path = format!("/repos/{}/{}/issues/{}/labels", repo.owner, repo.name, id.0);
            let _ = self.put::<Vec<GhLabel>, _>(&labels_path, &GhLabelsRequest { labels }).await;
        }

        Ok(MergeRequest::from(pr))
    }

    async fn get_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest> {
        let path = format!("/repos/{}/{}/pulls/{}", repo.owner, repo.name, id.0);
        let pr: GhPullRequest = self.get(&path).await?;
        Ok(MergeRequest::from(pr))
    }

    async fn list_mrs(
        &self,
        repo: &RepoId,
        filter: MergeRequestFilter,
    ) -> ProviderResult<Vec<MergeRequest>> {
        let mut query_parts = Vec::new();

        if let Some(state) = filter.state {
            let state_str = match state {
                MergeRequestState::Open | MergeRequestState::Draft => "open",
                MergeRequestState::Closed => "closed",
                MergeRequestState::Merged => "closed", // Need to filter by merged afterward
            };
            query_parts.push(format!("state={}", state_str));
        }

        if let Some(ref branch) = filter.source_branch {
            query_parts.push(format!("head={}:{}", repo.owner, branch));
        }

        if let Some(ref branch) = filter.target_branch {
            query_parts.push(format!("base={}", branch));
        }

        if let Some(limit) = filter.limit {
            query_parts.push(format!("per_page={}", limit.min(100)));
        }

        let query = if query_parts.is_empty() {
            String::new()
        } else {
            format!("?{}", query_parts.join("&"))
        };

        let path = format!("/repos/{}/{}/pulls{}", repo.owner, repo.name, query);
        let prs: Vec<GhPullRequest> = self.get(&path).await?;

        let mut mrs: Vec<MergeRequest> = prs.into_iter().map(MergeRequest::from).collect();

        // Filter by merged state if needed
        if filter.state == Some(MergeRequestState::Merged) {
            mrs.retain(|mr| mr.state == MergeRequestState::Merged);
        }

        // Filter by author if specified
        if let Some(ref author) = filter.author {
            mrs.retain(|mr| mr.author.as_ref() == Some(author));
        }

        // Filter by labels if specified
        if !filter.labels.is_empty() {
            mrs.retain(|mr| {
                filter.labels.iter().all(|l| mr.labels.contains(l))
            });
        }

        Ok(mrs)
    }

    async fn merge_mr(
        &self,
        repo: &RepoId,
        id: MergeRequestId,
        method: MergeMethod,
    ) -> ProviderResult<MergeResult> {
        let path = format!("/repos/{}/{}/pulls/{}/merge", repo.owner, repo.name, id.0);

        let method_str = match method {
            MergeMethod::Merge => "merge",
            MergeMethod::Squash => "squash",
            MergeMethod::Rebase => "rebase",
            MergeMethod::FastForward => {
                return Err(ProviderError::UnsupportedOperation(
                    "GitHub does not support fast-forward merge".to_string(),
                ));
            }
        };

        let body = GhMergeRequest {
            merge_method: method_str.to_string(),
        };

        let result: GhMergeResult = self.put(&path, &body).await?;

        Ok(MergeResult {
            merged: result.merged,
            sha: result.sha,
            message: result.message,
        })
    }

    async fn close_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest> {
        self.update_mr(
            repo,
            id,
            UpdateMergeRequest {
                state: Some(MergeRequestState::Closed),
                ..Default::default()
            },
        )
        .await
    }

    async fn reopen_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest> {
        self.update_mr(
            repo,
            id,
            UpdateMergeRequest {
                state: Some(MergeRequestState::Open),
                ..Default::default()
            },
        )
        .await
    }
}

#[async_trait]
impl UserProvider for GitHubProvider {
    async fn current_user(&self) -> ProviderResult<User> {
        let user: GhUser = self.get("/user").await?;
        Ok(User::from(user))
    }

    async fn validate_auth(&self) -> ProviderResult<bool> {
        match self.current_user().await {
            Ok(_) => Ok(true),
            Err(ProviderError::AuthenticationFailed(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get_user(&self, username: &str) -> ProviderResult<User> {
        let path = format!("/users/{}", username);
        let user: GhUser = self.get(&path).await?;
        Ok(User::from(user))
    }
}

#[async_trait]
impl RepositoryProvider for GitHubProvider {
    async fn check_access(&self, repo: &RepoId) -> ProviderResult<bool> {
        let path = format!("/repos/{}/{}", repo.owner, repo.name);
        match self.get::<GhRepository>(&path).await {
            Ok(_) => Ok(true),
            Err(ProviderError::NotFound(_)) => Ok(false),
            Err(ProviderError::AuthorizationDenied(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get_default_branch(&self, repo: &RepoId) -> ProviderResult<String> {
        let path = format!("/repos/{}/{}", repo.owner, repo.name);
        let repository: GhRepository = self.get(&path).await?;
        Ok(repository.default_branch)
    }

    fn parse_remote_url(&self, url: &str) -> Option<RepoId> {
        // Handle SSH URLs: git@github.com:owner/repo.git
        if let Some(rest) = url.strip_prefix("git@github.com:") {
            let repo_path = rest.trim_end_matches(".git");
            let parts: Vec<&str> = repo_path.split('/').collect();
            if parts.len() == 2 {
                return Some(RepoId::new(parts[0], parts[1]));
            }
        }

        // Handle HTTPS URLs: https://github.com/owner/repo.git
        if url.contains("github.com") {
            if let Ok(parsed) = url::Url::parse(url) {
                let path = parsed.path().trim_start_matches('/').trim_end_matches(".git");
                let parts: Vec<&str> = path.split('/').collect();
                if parts.len() == 2 {
                    return Some(RepoId::new(parts[0], parts[1]));
                }
            }
        }

        None
    }
}

#[async_trait]
impl PipelineProvider for GitHubProvider {
    async fn get_pipeline_status(
        &self,
        repo: &RepoId,
        ref_name: &str,
    ) -> ProviderResult<Option<Pipeline>> {
        let path = format!("/repos/{}/{}/commits/{}/check-runs", repo.owner, repo.name, ref_name);

        match self.get::<GhCheckRunsResponse>(&path).await {
            Ok(response) if response.total_count > 0 => {
                // Aggregate check runs into a pipeline-like status
                let mut overall_status = PipelineStatus::Success;
                let mut jobs = Vec::new();

                for run in response.check_runs {
                    let status = check_status_to_pipeline_status(&run.status, run.conclusion.as_deref());

                    // Overall status: Failed > Running > Pending > Success
                    if status == PipelineStatus::Failed {
                        overall_status = PipelineStatus::Failed;
                    } else if status == PipelineStatus::Running && overall_status != PipelineStatus::Failed {
                        overall_status = PipelineStatus::Running;
                    } else if status == PipelineStatus::Pending && overall_status == PipelineStatus::Success {
                        overall_status = PipelineStatus::Pending;
                    }

                    jobs.push(PipelineJob {
                        id: run.id,
                        name: run.name,
                        status,
                        web_url: run.html_url,
                    });
                }

                Ok(Some(Pipeline {
                    id: 0, // GitHub doesn't have a single pipeline ID
                    status: overall_status,
                    web_url: None,
                    ref_name: ref_name.to_string(),
                    sha: String::new(), // Would need another API call
                    created_at: Utc::now(), // Placeholder
                    jobs,
                }))
            }
            Ok(_) => Ok(None),
            Err(ProviderError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn list_mr_pipelines(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<Vec<Pipeline>> {
        // Get the PR to find the head branch
        let mr = self.get_mr(repo, mr_id).await?;

        // Get check runs for the head branch
        if let Some(pipeline) = self.get_pipeline_status(repo, &mr.source_branch).await? {
            Ok(vec![pipeline])
        } else {
            Ok(vec![])
        }
    }

    async fn trigger_pipeline(&self, _repo: &RepoId, _ref_name: &str) -> ProviderResult<Pipeline> {
        Err(ProviderError::UnsupportedOperation(
            "GitHub Actions cannot be manually triggered via this API".to_string(),
        ))
    }

    async fn cancel_pipeline(&self, _repo: &RepoId, _pipeline_id: u64) -> ProviderResult<()> {
        Err(ProviderError::UnsupportedOperation(
            "Cancelling GitHub Actions requires workflow run ID, not supported yet".to_string(),
        ))
    }

    async fn retry_pipeline(&self, _repo: &RepoId, _pipeline_id: u64) -> ProviderResult<Pipeline> {
        Err(ProviderError::UnsupportedOperation(
            "Retrying GitHub Actions requires workflow run ID, not supported yet".to_string(),
        ))
    }
}

#[async_trait]
impl ApprovalProvider for GitHubProvider {
    async fn list_reviews(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<Vec<Review>> {
        let path = format!("/repos/{}/{}/pulls/{}/reviews", repo.owner, repo.name, mr_id.0);
        let reviews: Vec<GhReview> = self.get(&path).await?;
        Ok(reviews.into_iter().map(Review::from).collect())
    }

    async fn request_review(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        reviewers: Vec<String>,
    ) -> ProviderResult<()> {
        let path = format!("/repos/{}/{}/pulls/{}/requested_reviewers", repo.owner, repo.name, mr_id.0);
        let _: serde_json::Value = self.post(&path, &GhReviewersRequest { reviewers }).await?;
        Ok(())
    }

    async fn get_approval_status(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<ApprovalState> {
        let reviews = self.list_reviews(repo, mr_id).await?;

        // Aggregate reviews by user, keeping only the latest review per user
        use std::collections::HashMap;
        let mut latest_by_user: HashMap<String, ApprovalState> = HashMap::new();

        for review in reviews {
            latest_by_user.insert(review.user, review.state);
        }

        // Determine overall status
        let has_changes_requested = latest_by_user.values().any(|s| *s == ApprovalState::ChangesRequested);
        let has_approval = latest_by_user.values().any(|s| *s == ApprovalState::Approved);

        if has_changes_requested {
            Ok(ApprovalState::ChangesRequested)
        } else if has_approval {
            Ok(ApprovalState::Approved)
        } else {
            Ok(ApprovalState::Pending)
        }
    }

    async fn has_required_approvals(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<bool> {
        // GitHub's branch protection rules determine required approvals
        // This would need the branch protection API to check accurately
        // For now, just check if there's at least one approval
        let status = self.get_approval_status(repo, mr_id).await?;
        Ok(status == ApprovalState::Approved)
    }
}

#[async_trait]
impl LabelProvider for GitHubProvider {
    async fn list_labels(&self, repo: &RepoId) -> ProviderResult<Vec<Label>> {
        let path = format!("/repos/{}/{}/labels", repo.owner, repo.name);
        let labels: Vec<GhLabel> = self.get(&path).await?;
        Ok(labels.into_iter().map(Label::from).collect())
    }

    async fn add_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()> {
        let path = format!("/repos/{}/{}/issues/{}/labels", repo.owner, repo.name, mr_id.0);
        let _: Vec<GhLabel> = self.post(&path, &GhLabelsRequest { labels }).await?;
        Ok(())
    }

    async fn remove_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()> {
        for label in labels {
            let path = format!("/repos/{}/{}/issues/{}/labels/{}", repo.owner, repo.name, mr_id.0, label);
            let _ = self.delete(&path).await;
        }
        Ok(())
    }

    async fn set_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()> {
        let path = format!("/repos/{}/{}/issues/{}/labels", repo.owner, repo.name, mr_id.0);
        let _: Vec<GhLabel> = self.put(&path, &GhLabelsRequest { labels }).await?;
        Ok(())
    }
}

#[async_trait]
impl MilestoneProvider for GitHubProvider {
    async fn list_milestones(
        &self,
        repo: &RepoId,
        state: Option<MilestoneState>,
    ) -> ProviderResult<Vec<Milestone>> {
        let state_param = match state {
            Some(MilestoneState::Open) => "?state=open",
            Some(MilestoneState::Closed) => "?state=closed",
            None => "?state=all",
        };

        let path = format!("/repos/{}/{}/milestones{}", repo.owner, repo.name, state_param);
        let milestones: Vec<GhMilestone> = self.get(&path).await?;
        Ok(milestones.into_iter().map(Milestone::from).collect())
    }

    async fn assign_milestone(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        milestone_id: u64,
    ) -> ProviderResult<()> {
        let path = format!("/repos/{}/{}/issues/{}", repo.owner, repo.name, mr_id.0);
        let body = serde_json::json!({ "milestone": milestone_id });
        let _: serde_json::Value = self.patch(&path, &body).await?;
        Ok(())
    }

    async fn remove_milestone(&self, repo: &RepoId, mr_id: MergeRequestId) -> ProviderResult<()> {
        let path = format!("/repos/{}/{}/issues/{}", repo.owner, repo.name, mr_id.0);
        let body = serde_json::json!({ "milestone": null });
        let _: serde_json::Value = self.patch(&path, &body).await?;
        Ok(())
    }
}

impl Provider for GitHubProvider {
    fn name(&self) -> &'static str {
        "github"
    }

    fn display_name(&self) -> &'static str {
        "GitHub"
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            merge_requests: true,
            pipelines: true,
            approvals: true,
            labels: true,
            milestones: true,
            draft_mrs: true,
            squash_merge: true,
            rebase_merge: true,
            fast_forward_merge: false, // GitHub doesn't support this
            branch_protection: true,
        }
    }

    fn pipelines(&self) -> Option<&dyn PipelineProvider> {
        Some(self)
    }

    fn approvals(&self) -> Option<&dyn ApprovalProvider> {
        Some(self)
    }

    fn labels(&self) -> Option<&dyn LabelProvider> {
        Some(self)
    }

    fn milestones(&self) -> Option<&dyn MilestoneProvider> {
        Some(self)
    }

    fn branch_protection(&self) -> Option<&dyn BranchProtectionProvider> {
        Some(self)
    }
}

// Branch protection implementation
#[async_trait]
impl BranchProtectionProvider for GitHubProvider {
    async fn get_branch_protection(
        &self,
        repo: &RepoId,
        branch: &str,
    ) -> ProviderResult<Option<BranchProtection>> {
        let url = format!(
            "{}/repos/{}/{}/branches/{}/protection",
            self.base_url,
            repo.owner,
            repo.name,
            branch
        );

        // GitHub returns 404 if branch is not protected
        match self.get::<serde_json::Value>(&url).await {
            Ok(json) => {
                let protection = BranchProtection {
                    pattern: branch.to_string(),
                    is_protected: true,
                    require_pull_request: json
                        .get("required_pull_request_reviews")
                        .is_some(),
                    required_approvals: json
                        .get("required_pull_request_reviews")
                        .and_then(|r| r.get("required_approving_review_count"))
                        .and_then(|c| c.as_u64())
                        .unwrap_or(0) as u32,
                    require_status_checks: json
                        .get("required_status_checks")
                        .is_some(),
                    require_linear_history: json
                        .get("required_linear_history")
                        .and_then(|r| r.get("enabled"))
                        .and_then(|e| e.as_bool())
                        .unwrap_or(false),
                    allow_force_push: json
                        .get("allow_force_pushes")
                        .and_then(|r| r.get("enabled"))
                        .and_then(|e| e.as_bool())
                        .unwrap_or(false),
                    allow_deletions: json
                        .get("allow_deletions")
                        .and_then(|r| r.get("enabled"))
                        .and_then(|e| e.as_bool())
                        .unwrap_or(false),
                };
                Ok(Some(protection))
            }
            Err(ProviderError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_remote_url_ssh() {
        let provider = GitHubProvider::new("test").unwrap();

        let repo = provider.parse_remote_url("git@github.com:owner/repo.git");
        assert!(repo.is_some());
        let repo = repo.unwrap();
        assert_eq!(repo.owner, "owner");
        assert_eq!(repo.name, "repo");
    }

    #[test]
    fn test_parse_remote_url_https() {
        let provider = GitHubProvider::new("test").unwrap();

        let repo = provider.parse_remote_url("https://github.com/owner/repo.git");
        assert!(repo.is_some());
        let repo = repo.unwrap();
        assert_eq!(repo.owner, "owner");
        assert_eq!(repo.name, "repo");
    }

    #[test]
    fn test_parse_remote_url_invalid() {
        let provider = GitHubProvider::new("test").unwrap();

        assert!(provider.parse_remote_url("git@gitlab.com:owner/repo.git").is_none());
        assert!(provider.parse_remote_url("https://gitlab.com/owner/repo").is_none());
    }
}
