//! GitLab provider implementation.
//!
//! This module implements the provider traits for GitLab, enabling
//! Stack to work with GitLab merge requests, pipelines, and other features.
//!
//! # Example
//!
//! ```rust,ignore
//! use stkd_gitlab::GitLabProvider;
//! use stkd_provider_api::{Provider, RepoId, MergeRequestProvider};
//!
//! // Create provider with personal access token
//! let provider = GitLabProvider::new("glpat-xxxxxxxxxxxxxxxxxxxx")?;
//!
//! // Or with a custom host for self-hosted GitLab
//! let provider = GitLabProvider::with_host("glpat-xxx", "gitlab.mycompany.com")?;
//!
//! // Use provider traits
//! let repo = RepoId::new("group", "project");
//! let mr = provider.get_mr(&repo, 42.into()).await?;
//! ```

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};
use url::Url;

use crate::auth::GitLabAuth;
use stkd_provider_api::{
    ApprovalProvider, ApprovalState, BranchProtection, BranchProtectionProvider,
    CreateMergeRequest, Label, LabelProvider, MergeMethod, MergeRequest, MergeRequestFilter,
    MergeRequestId, MergeRequestProvider, MergeRequestState, MergeResult, Milestone,
    MilestoneProvider, MilestoneState, Pipeline, PipelineProvider, PipelineStatus, Provider,
    ProviderCapabilities, ProviderError, ProviderResult, RepoId, RepositoryProvider, Review,
    UpdateMergeRequest, User, UserProvider,
};

/// Default GitLab host.
const DEFAULT_HOST: &str = "gitlab.com";

/// GitLab API version.
const API_VERSION: &str = "v4";

/// GitLab provider implementing all provider traits.
///
/// This provider supports:
/// - GitLab.com and self-hosted GitLab instances
/// - Personal access tokens, OAuth tokens, and job tokens
/// - All merge request operations
/// - Pipeline status and management
/// - Approvals and reviews
/// - Labels and milestones
#[derive(Debug, Clone)]
pub struct GitLabProvider {
    client: Client,
    auth: GitLabAuth,
    host: String,
    api_url: String,
}

impl GitLabProvider {
    /// Create a new GitLab provider with a personal access token.
    ///
    /// Uses gitlab.com by default.
    ///
    /// # Arguments
    ///
    /// * `token` - GitLab personal access token (glpat-...)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let provider = GitLabProvider::new("glpat-xxxxxxxxxxxxxxxxxxxx")?;
    /// ```
    pub fn new(token: impl Into<String>) -> ProviderResult<Self> {
        Self::with_host(token, DEFAULT_HOST)
    }

    /// Create a new GitLab provider with a custom host.
    ///
    /// # Arguments
    ///
    /// * `token` - GitLab personal access token
    /// * `host` - GitLab host (e.g., "gitlab.mycompany.com")
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let provider = GitLabProvider::with_host("glpat-xxx", "gitlab.mycompany.com")?;
    /// ```
    pub fn with_host(token: impl Into<String>, host: impl Into<String>) -> ProviderResult<Self> {
        let host = host.into();
        let api_url = format!("https://{}/api/{}", host, API_VERSION);

        let client = Client::builder()
            .user_agent("stkd-cli")
            .build()
            .map_err(|e| ProviderError::Internal(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            auth: GitLabAuth::PersonalAccessToken(token.into()),
            host,
            api_url,
        })
    }

    /// Create a provider with OAuth authentication.
    pub fn with_oauth(token: impl Into<String>, host: impl Into<String>) -> ProviderResult<Self> {
        let mut provider = Self::with_host("placeholder", host)?;
        provider.auth = GitLabAuth::OAuth(token.into());
        Ok(provider)
    }

    /// Create a provider with a CI job token.
    pub fn with_job_token(
        token: impl Into<String>,
        host: impl Into<String>,
    ) -> ProviderResult<Self> {
        let mut provider = Self::with_host("placeholder", host)?;
        provider.auth = GitLabAuth::JobToken(token.into());
        Ok(provider)
    }

    /// Get the API base URL.
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Get the host name.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Build a URL for a project endpoint.
    fn project_url(&self, repo: &RepoId, path: &str) -> String {
        let project_path = format!("{}/{}", repo.owner, repo.name);
        let encoded = urlencoding::encode(&project_path);
        format!("{}/projects/{}{}", self.api_url, encoded, path)
    }

    /// Execute an API request with authentication.
    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> ProviderResult<T> {
        self.request_with_body(method, url, None::<()>).await
    }

    /// Execute an API request with authentication and optional body.
    async fn request_with_body<T: for<'de> Deserialize<'de>, B: Serialize>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<B>,
    ) -> ProviderResult<T> {
        trace!("GitLab API: {} {}", method, url);

        let mut req = self
            .client
            .request(method.clone(), url)
            .header(self.auth.header_name(), self.auth.header_value());

        if let Some(body) = body {
            req = req.json(&body);
        }

        let response = req
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        let status = response.status();

        // Handle error responses
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(self.handle_error(status, &body));
        }

        // Parse successful response
        let text = response
            .text()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        trace!("GitLab API response: {}", text);

        serde_json::from_str(&text).map_err(|e| {
            ProviderError::Internal(format!("Failed to parse response: {} - {}", e, text))
        })
    }

    /// Execute a request that returns no content.
    async fn request_no_content(&self, method: reqwest::Method, url: &str) -> ProviderResult<()> {
        trace!("GitLab API: {} {}", method, url);

        let response = self
            .client
            .request(method.clone(), url)
            .header(self.auth.header_name(), self.auth.header_value())
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        let status = response.status();

        if !status.is_success() && status != StatusCode::NO_CONTENT {
            let body = response.text().await.unwrap_or_default();
            return Err(self.handle_error(status, &body));
        }

        Ok(())
    }

    /// Handle HTTP error responses.
    fn handle_error(&self, status: StatusCode, body: &str) -> ProviderError {
        debug!("GitLab API error {}: {}", status, body);

        match status {
            StatusCode::UNAUTHORIZED => {
                ProviderError::AuthenticationFailed("Invalid or expired token".to_string())
            }
            StatusCode::FORBIDDEN => {
                ProviderError::AuthorizationDenied("Access denied".to_string())
            }
            StatusCode::NOT_FOUND => ProviderError::NotFound("Resource not found".to_string()),
            StatusCode::UNPROCESSABLE_ENTITY => {
                // Try to extract error message from response
                let message = serde_json::from_str::<GlErrorResponse>(body)
                    .map(|e| e.message.unwrap_or_else(|| body.to_string()))
                    .unwrap_or_else(|_| body.to_string());
                ProviderError::ValidationError(message)
            }
            StatusCode::TOO_MANY_REQUESTS => {
                // Try to parse retry-after from response
                ProviderError::RateLimited { retry_after: None }
            }
            StatusCode::CONFLICT => ProviderError::MergeConflict("Resource conflict".to_string()),
            _ => ProviderError::ProviderSpecific(format!("HTTP {}: {}", status, body)),
        }
    }
}

// GitLab API response types

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GlErrorResponse {
    message: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GlUser {
    id: u64,
    username: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,
}

impl From<GlUser> for User {
    fn from(u: GlUser) -> Self {
        User {
            id: u.id,
            username: u.username,
            name: u.name,
            email: u.email,
            avatar_url: u.avatar_url,
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GlProject {
    id: u64,
    path_with_namespace: String,
    default_branch: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GlMergeRequest {
    id: u64,
    iid: u64,
    title: String,
    description: Option<String>,
    state: String,
    source_branch: String,
    target_branch: String,
    web_url: String,
    work_in_progress: Option<bool>,
    draft: Option<bool>,
    merge_status: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    author: Option<GlAuthor>,
    labels: Option<Vec<String>>,
    milestone: Option<GlMilestone>,
}

#[derive(Debug, Deserialize)]
struct GlAuthor {
    username: String,
}

#[derive(Debug, Deserialize)]
struct GlMilestone {
    id: u64,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    state: String,
}

impl From<GlMergeRequest> for MergeRequest {
    fn from(mr: GlMergeRequest) -> Self {
        let is_draft = mr.draft.unwrap_or(false) || mr.work_in_progress.unwrap_or(false);
        let state = match mr.state.as_str() {
            "opened" => {
                if is_draft {
                    MergeRequestState::Draft
                } else {
                    MergeRequestState::Open
                }
            }
            "closed" => MergeRequestState::Closed,
            "merged" => MergeRequestState::Merged,
            _ => MergeRequestState::Open,
        };

        let mergeable = mr.merge_status.as_deref().map(|s| s == "can_be_merged");

        MergeRequest {
            id: MergeRequestId(mr.iid),
            number: mr.iid,
            title: mr.title,
            body: mr.description,
            state,
            source_branch: mr.source_branch,
            target_branch: mr.target_branch,
            web_url: mr.web_url,
            is_draft,
            mergeable,
            created_at: mr.created_at,
            updated_at: mr.updated_at,
            author: mr.author.map(|a| a.username),
            labels: mr.labels.unwrap_or_default(),
            milestone: mr.milestone.map(|m| m.title),
        }
    }
}

#[derive(Debug, Serialize)]
struct CreateMrRequest {
    source_branch: String,
    target_branch: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewer_ids: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    draft: bool,
}

#[derive(Debug, Serialize)]
struct UpdateMrRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignee_ids: Option<Vec<u64>>,
}

#[derive(Debug, Serialize)]
struct MergeMrRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_commit_message: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    squash: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    should_remove_source_branch: bool,
}

#[derive(Debug, Deserialize)]
struct GlMergeResult {
    state: String,
    merge_commit_sha: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GlPipeline {
    id: u64,
    status: String,
    web_url: Option<String>,
    #[serde(rename = "ref")]
    ref_name: String,
    sha: String,
    created_at: DateTime<Utc>,
}

// Reserved for future use when we want to fetch pipeline jobs
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GlJob {
    id: u64,
    name: String,
    status: String,
    web_url: Option<String>,
}

fn parse_pipeline_status(status: &str) -> PipelineStatus {
    match status {
        "pending" | "created" | "waiting_for_resource" | "preparing" | "scheduled" => {
            PipelineStatus::Pending
        }
        "running" => PipelineStatus::Running,
        "success" => PipelineStatus::Success,
        "failed" => PipelineStatus::Failed,
        "canceled" | "cancelled" => PipelineStatus::Canceled,
        "skipped" => PipelineStatus::Skipped,
        _ => PipelineStatus::Unknown,
    }
}

impl From<GlPipeline> for Pipeline {
    fn from(p: GlPipeline) -> Self {
        Pipeline {
            id: p.id,
            status: parse_pipeline_status(&p.status),
            web_url: p.web_url,
            ref_name: p.ref_name,
            sha: p.sha,
            created_at: p.created_at,
            jobs: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GlApproval {
    user: GlUser,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GlApprovalState {
    approved: bool,
    approved_by: Vec<GlApproval>,
    approvals_required: Option<u32>,
    approvals_left: Option<u32>,
}

// Reserved for future use to fetch MR notes/comments
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GlNote {
    id: u64,
    author: GlUser,
    body: String,
    system: bool,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GlLabel {
    id: u64,
    name: String,
    color: Option<String>,
    description: Option<String>,
}

impl From<GlLabel> for Label {
    fn from(l: GlLabel) -> Self {
        Label {
            name: l.name,
            color: l.color.map(|c| c.trim_start_matches('#').to_string()),
            description: l.description,
        }
    }
}

impl From<GlMilestone> for Milestone {
    fn from(m: GlMilestone) -> Self {
        let state = match m.state.as_str() {
            "active" => MilestoneState::Open,
            "closed" => MilestoneState::Closed,
            _ => MilestoneState::Open,
        };

        Milestone {
            id: m.id,
            title: m.title,
            description: m.description,
            due_date: m.due_date.and_then(|d| d.parse().ok()),
            state,
        }
    }
}

// Trait implementations

#[async_trait]
impl MergeRequestProvider for GitLabProvider {
    async fn create_mr(
        &self,
        repo: &RepoId,
        request: CreateMergeRequest,
    ) -> ProviderResult<MergeRequest> {
        let url = self.project_url(repo, "/merge_requests");

        let body = CreateMrRequest {
            source_branch: request.source_branch,
            target_branch: request.target_branch,
            title: request.title,
            description: request.body,
            labels: if request.labels.is_empty() {
                None
            } else {
                Some(request.labels.join(","))
            },
            assignee_ids: None, // Would need user ID lookup
            reviewer_ids: None, // Would need user ID lookup
            draft: request.draft,
        };

        let mr: GlMergeRequest = self
            .request_with_body(reqwest::Method::POST, &url, Some(&body))
            .await?;

        Ok(mr.into())
    }

    async fn update_mr(
        &self,
        repo: &RepoId,
        id: MergeRequestId,
        update: UpdateMergeRequest,
    ) -> ProviderResult<MergeRequest> {
        let url = self.project_url(repo, &format!("/merge_requests/{}", id.0));

        let state_event = update.state.map(|s| match s {
            MergeRequestState::Closed => "close".to_string(),
            MergeRequestState::Open | MergeRequestState::Draft => "reopen".to_string(),
            _ => "reopen".to_string(),
        });

        let body = UpdateMrRequest {
            title: update.title,
            description: update.body,
            target_branch: update.target_branch,
            state_event,
            labels: update.labels.map(|l| l.join(",")),
            assignee_ids: None, // Would need user ID lookup
        };

        let mr: GlMergeRequest = self
            .request_with_body(reqwest::Method::PUT, &url, Some(&body))
            .await?;

        Ok(mr.into())
    }

    async fn get_mr(&self, repo: &RepoId, id: MergeRequestId) -> ProviderResult<MergeRequest> {
        let url = self.project_url(repo, &format!("/merge_requests/{}", id.0));
        let mr: GlMergeRequest = self.request(reqwest::Method::GET, &url).await?;
        Ok(mr.into())
    }

    async fn list_mrs(
        &self,
        repo: &RepoId,
        filter: MergeRequestFilter,
    ) -> ProviderResult<Vec<MergeRequest>> {
        let mut url = self.project_url(repo, "/merge_requests?");

        let mut params = Vec::new();

        if let Some(state) = filter.state {
            let state_str = match state {
                MergeRequestState::Open | MergeRequestState::Draft => "opened",
                MergeRequestState::Closed => "closed",
                MergeRequestState::Merged => "merged",
            };
            params.push(format!("state={}", state_str));
        }

        if let Some(source) = filter.source_branch {
            params.push(format!("source_branch={}", urlencoding::encode(&source)));
        }

        if let Some(target) = filter.target_branch {
            params.push(format!("target_branch={}", urlencoding::encode(&target)));
        }

        if let Some(author) = filter.author {
            params.push(format!("author_username={}", urlencoding::encode(&author)));
        }

        if !filter.labels.is_empty() {
            params.push(format!("labels={}", filter.labels.join(",")));
        }

        if let Some(limit) = filter.limit {
            params.push(format!("per_page={}", limit));
        }

        url.push_str(&params.join("&"));

        let mrs: Vec<GlMergeRequest> = self.request(reqwest::Method::GET, &url).await?;
        Ok(mrs.into_iter().map(|mr| mr.into()).collect())
    }

    async fn merge_mr(
        &self,
        repo: &RepoId,
        id: MergeRequestId,
        method: MergeMethod,
    ) -> ProviderResult<MergeResult> {
        // Check if fast-forward is requested (GitLab specific)
        if method == MergeMethod::FastForward {
            // For fast-forward, we need to check if the MR can be merged with ff only
            let url = self.project_url(repo, &format!("/merge_requests/{}/merge", id.0));

            #[derive(Serialize)]
            struct FfMergeRequest {
                merge_method: String,
            }

            let body = FfMergeRequest {
                merge_method: "ff".to_string(),
            };

            match self
                .request_with_body::<GlMergeResult, _>(reqwest::Method::PUT, &url, Some(&body))
                .await
            {
                Ok(result) => {
                    return Ok(MergeResult {
                        merged: result.state == "merged",
                        sha: result.merge_commit_sha,
                        message: "Merge request merged".to_string(),
                    });
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let url = self.project_url(repo, &format!("/merge_requests/{}/merge", id.0));

        let body = MergeMrRequest {
            merge_commit_message: None,
            squash: method == MergeMethod::Squash,
            should_remove_source_branch: false,
        };

        let result: GlMergeResult = self
            .request_with_body(reqwest::Method::PUT, &url, Some(&body))
            .await?;

        Ok(MergeResult {
            merged: result.state == "merged",
            sha: result.merge_commit_sha,
            message: "Merge request merged".to_string(),
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
impl UserProvider for GitLabProvider {
    async fn current_user(&self) -> ProviderResult<User> {
        let url = format!("{}/user", self.api_url);
        let user: GlUser = self.request(reqwest::Method::GET, &url).await?;
        Ok(user.into())
    }

    async fn validate_auth(&self) -> ProviderResult<bool> {
        match self.current_user().await {
            Ok(_) => Ok(true),
            Err(ProviderError::AuthenticationFailed(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get_user(&self, username: &str) -> ProviderResult<User> {
        let url = format!(
            "{}/users?username={}",
            self.api_url,
            urlencoding::encode(username)
        );
        let users: Vec<GlUser> = self.request(reqwest::Method::GET, &url).await?;
        users
            .into_iter()
            .next()
            .map(|u| u.into())
            .ok_or_else(|| ProviderError::NotFound(format!("User '{}' not found", username)))
    }
}

#[async_trait]
impl RepositoryProvider for GitLabProvider {
    async fn check_access(&self, repo: &RepoId) -> ProviderResult<bool> {
        let url = self.project_url(repo, "");
        match self.request::<GlProject>(reqwest::Method::GET, &url).await {
            Ok(_) => Ok(true),
            Err(ProviderError::NotFound(_)) => Ok(false),
            Err(ProviderError::AuthorizationDenied(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn get_default_branch(&self, repo: &RepoId) -> ProviderResult<String> {
        let url = self.project_url(repo, "");
        let project: GlProject = self.request(reqwest::Method::GET, &url).await?;
        Ok(project.default_branch.unwrap_or_else(|| "main".to_string()))
    }

    fn parse_remote_url(&self, url: &str) -> Option<RepoId> {
        // Try to parse GitLab URLs in various formats:
        // - https://gitlab.com/group/project.git
        // - git@gitlab.com:group/project.git
        // - https://gitlab.mycompany.com/group/subgroup/project

        // SSH format
        if let Some(rest) = url.strip_prefix(&format!("git@{}:", self.host)) {
            let path = rest.trim_end_matches(".git");
            return parse_gitlab_path(path);
        }

        // HTTPS format
        if let Ok(parsed) = Url::parse(url) {
            if parsed.host_str() == Some(&self.host) {
                let path = parsed
                    .path()
                    .trim_start_matches('/')
                    .trim_end_matches(".git");
                return parse_gitlab_path(path);
            }
        }

        None
    }
}

/// Parse a GitLab project path into RepoId.
///
/// GitLab paths can include subgroups: group/subgroup/project
/// We treat everything except the last component as the "owner".
fn parse_gitlab_path(path: &str) -> Option<RepoId> {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 2 {
        let project = parts.last()?.to_string();
        let owner = parts[..parts.len() - 1].join("/");
        Some(RepoId::new(owner, project))
    } else {
        None
    }
}

#[async_trait]
impl PipelineProvider for GitLabProvider {
    async fn get_pipeline_status(
        &self,
        repo: &RepoId,
        ref_name: &str,
    ) -> ProviderResult<Option<Pipeline>> {
        let url = self.project_url(
            repo,
            &format!(
                "/pipelines?ref={}&per_page=1",
                urlencoding::encode(ref_name)
            ),
        );

        let pipelines: Vec<GlPipeline> = self.request(reqwest::Method::GET, &url).await?;
        Ok(pipelines.into_iter().next().map(|p| p.into()))
    }

    async fn list_mr_pipelines(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<Vec<Pipeline>> {
        let url = self.project_url(repo, &format!("/merge_requests/{}/pipelines", mr_id.0));
        let pipelines: Vec<GlPipeline> = self.request(reqwest::Method::GET, &url).await?;
        Ok(pipelines.into_iter().map(|p| p.into()).collect())
    }

    async fn trigger_pipeline(&self, repo: &RepoId, ref_name: &str) -> ProviderResult<Pipeline> {
        let url = self.project_url(repo, "/pipeline");

        #[derive(Serialize)]
        struct TriggerRequest<'a> {
            #[serde(rename = "ref")]
            ref_name: &'a str,
        }

        let body = TriggerRequest { ref_name };
        let pipeline: GlPipeline = self
            .request_with_body(reqwest::Method::POST, &url, Some(&body))
            .await?;
        Ok(pipeline.into())
    }

    async fn cancel_pipeline(&self, repo: &RepoId, pipeline_id: u64) -> ProviderResult<()> {
        let url = self.project_url(repo, &format!("/pipelines/{}/cancel", pipeline_id));
        self.request_no_content(reqwest::Method::POST, &url).await
    }

    async fn retry_pipeline(&self, repo: &RepoId, pipeline_id: u64) -> ProviderResult<Pipeline> {
        let url = self.project_url(repo, &format!("/pipelines/{}/retry", pipeline_id));
        let pipeline: GlPipeline = self.request(reqwest::Method::POST, &url).await?;
        Ok(pipeline.into())
    }
}

#[async_trait]
impl ApprovalProvider for GitLabProvider {
    async fn list_reviews(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<Vec<Review>> {
        // GitLab uses approvals API instead of reviews
        let url = self.project_url(repo, &format!("/merge_requests/{}/approvals", mr_id.0));
        let approvals: GlApprovalState = self.request(reqwest::Method::GET, &url).await?;

        let reviews = approvals
            .approved_by
            .into_iter()
            .enumerate()
            .map(|(i, a)| Review {
                id: i as u64,
                user: a.user.username,
                state: ApprovalState::Approved,
                body: None,
                submitted_at: None,
            })
            .collect();

        Ok(reviews)
    }

    async fn request_review(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        reviewers: Vec<String>,
    ) -> ProviderResult<()> {
        // GitLab requires user IDs, not usernames
        // First, look up user IDs
        let mut reviewer_ids = Vec::new();
        for username in &reviewers {
            let user = self.get_user(username).await?;
            reviewer_ids.push(user.id);
        }

        let url = self.project_url(repo, &format!("/merge_requests/{}", mr_id.0));

        #[derive(Serialize)]
        struct UpdateReviewers {
            reviewer_ids: Vec<u64>,
        }

        let body = UpdateReviewers { reviewer_ids };
        self.request_with_body::<GlMergeRequest, _>(reqwest::Method::PUT, &url, Some(&body))
            .await?;
        Ok(())
    }

    async fn get_approval_status(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
    ) -> ProviderResult<ApprovalState> {
        let url = self.project_url(repo, &format!("/merge_requests/{}/approvals", mr_id.0));
        let approvals: GlApprovalState = self.request(reqwest::Method::GET, &url).await?;

        if approvals.approved {
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
        let url = self.project_url(repo, &format!("/merge_requests/{}/approvals", mr_id.0));
        let approvals: GlApprovalState = self.request(reqwest::Method::GET, &url).await?;
        Ok(approvals.approved)
    }
}

#[async_trait]
impl LabelProvider for GitLabProvider {
    async fn list_labels(&self, repo: &RepoId) -> ProviderResult<Vec<Label>> {
        let url = self.project_url(repo, "/labels?per_page=100");
        let labels: Vec<GlLabel> = self.request(reqwest::Method::GET, &url).await?;
        Ok(labels.into_iter().map(|l| l.into()).collect())
    }

    async fn add_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()> {
        // Get current labels first
        let mr = self.get_mr(repo, mr_id).await?;
        let mut all_labels = mr.labels;
        all_labels.extend(labels);

        // Update with all labels
        let url = self.project_url(repo, &format!("/merge_requests/{}", mr_id.0));

        #[derive(Serialize)]
        struct UpdateLabels {
            labels: String,
        }

        let body = UpdateLabels {
            labels: all_labels.join(","),
        };
        self.request_with_body::<GlMergeRequest, _>(reqwest::Method::PUT, &url, Some(&body))
            .await?;
        Ok(())
    }

    async fn remove_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()> {
        // Get current labels first
        let mr = self.get_mr(repo, mr_id).await?;
        let remaining_labels: Vec<String> = mr
            .labels
            .into_iter()
            .filter(|l| !labels.contains(l))
            .collect();

        // Update with remaining labels
        let url = self.project_url(repo, &format!("/merge_requests/{}", mr_id.0));

        #[derive(Serialize)]
        struct UpdateLabels {
            labels: String,
        }

        let body = UpdateLabels {
            labels: remaining_labels.join(","),
        };
        self.request_with_body::<GlMergeRequest, _>(reqwest::Method::PUT, &url, Some(&body))
            .await?;
        Ok(())
    }

    async fn set_labels(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        labels: Vec<String>,
    ) -> ProviderResult<()> {
        let url = self.project_url(repo, &format!("/merge_requests/{}", mr_id.0));

        #[derive(Serialize)]
        struct UpdateLabels {
            labels: String,
        }

        let body = UpdateLabels {
            labels: labels.join(","),
        };
        self.request_with_body::<GlMergeRequest, _>(reqwest::Method::PUT, &url, Some(&body))
            .await?;
        Ok(())
    }
}

#[async_trait]
impl MilestoneProvider for GitLabProvider {
    async fn list_milestones(
        &self,
        repo: &RepoId,
        state: Option<MilestoneState>,
    ) -> ProviderResult<Vec<Milestone>> {
        let mut url = self.project_url(repo, "/milestones?per_page=100");

        if let Some(state) = state {
            let state_str = match state {
                MilestoneState::Open => "active",
                MilestoneState::Closed => "closed",
            };
            url.push_str(&format!("&state={}", state_str));
        }

        let milestones: Vec<GlMilestone> = self.request(reqwest::Method::GET, &url).await?;
        Ok(milestones.into_iter().map(|m| m.into()).collect())
    }

    async fn assign_milestone(
        &self,
        repo: &RepoId,
        mr_id: MergeRequestId,
        milestone_id: u64,
    ) -> ProviderResult<()> {
        let url = self.project_url(repo, &format!("/merge_requests/{}", mr_id.0));

        #[derive(Serialize)]
        struct UpdateMilestone {
            milestone_id: u64,
        }

        let body = UpdateMilestone { milestone_id };
        self.request_with_body::<GlMergeRequest, _>(reqwest::Method::PUT, &url, Some(&body))
            .await?;
        Ok(())
    }

    async fn remove_milestone(&self, repo: &RepoId, mr_id: MergeRequestId) -> ProviderResult<()> {
        let url = self.project_url(repo, &format!("/merge_requests/{}", mr_id.0));

        #[derive(Serialize)]
        struct UpdateMilestone {
            milestone_id: Option<u64>,
        }

        let body = UpdateMilestone { milestone_id: None };
        self.request_with_body::<GlMergeRequest, _>(reqwest::Method::PUT, &url, Some(&body))
            .await?;
        Ok(())
    }
}

impl Provider for GitLabProvider {
    fn name(&self) -> &'static str {
        "gitlab"
    }

    fn display_name(&self) -> &'static str {
        "GitLab"
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
            fast_forward_merge: true,
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
impl BranchProtectionProvider for GitLabProvider {
    async fn get_branch_protection(
        &self,
        repo: &RepoId,
        branch: &str,
    ) -> ProviderResult<Option<BranchProtection>> {
        let url = self.project_url(
            repo,
            &format!("/protected_branches/{}", urlencoding::encode(branch)),
        );

        // GitLab returns 404 if branch is not protected
        match self
            .request::<serde_json::Value>(reqwest::Method::GET, &url)
            .await
        {
            Ok(json) => {
                let protection = BranchProtection {
                    pattern: json
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or(branch)
                        .to_string(),
                    is_protected: true,
                    require_pull_request: true, // Protected branches in GitLab require MRs by default
                    required_approvals: json
                        .get("merge_access_levels")
                        .and_then(|m| m.as_array())
                        .map(|arr| if arr.is_empty() { 0 } else { 1 })
                        .unwrap_or(0),
                    require_status_checks: false, // GitLab handles this differently
                    require_linear_history: false, // Not a direct GitLab concept
                    allow_force_push: json
                        .get("allow_force_push")
                        .and_then(|a| a.as_bool())
                        .unwrap_or(false),
                    allow_deletions: false, // Protected branches can't be deleted by default
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
    fn test_parse_gitlab_path() {
        let repo = parse_gitlab_path("group/project").unwrap();
        assert_eq!(repo.owner, "group");
        assert_eq!(repo.name, "project");

        let repo = parse_gitlab_path("group/subgroup/project").unwrap();
        assert_eq!(repo.owner, "group/subgroup");
        assert_eq!(repo.name, "project");

        let repo = parse_gitlab_path("a/b/c/d").unwrap();
        assert_eq!(repo.owner, "a/b/c");
        assert_eq!(repo.name, "d");

        assert!(parse_gitlab_path("project").is_none());
    }

    #[test]
    fn test_parse_remote_url() {
        let provider = GitLabProvider::new("token").unwrap();

        // HTTPS URL
        let repo = provider
            .parse_remote_url("https://gitlab.com/group/project.git")
            .unwrap();
        assert_eq!(repo.owner, "group");
        assert_eq!(repo.name, "project");

        // SSH URL
        let repo = provider
            .parse_remote_url("git@gitlab.com:group/project.git")
            .unwrap();
        assert_eq!(repo.owner, "group");
        assert_eq!(repo.name, "project");

        // Subgroup
        let repo = provider
            .parse_remote_url("https://gitlab.com/group/subgroup/project")
            .unwrap();
        assert_eq!(repo.owner, "group/subgroup");
        assert_eq!(repo.name, "project");

        // Wrong host
        assert!(provider
            .parse_remote_url("https://github.com/owner/repo.git")
            .is_none());
    }

    #[test]
    fn test_project_url() {
        let provider = GitLabProvider::new("token").unwrap();
        let repo = RepoId::new("group", "project");

        let url = provider.project_url(&repo, "/merge_requests");
        assert!(url.contains("projects/group%2Fproject/merge_requests"));

        // With subgroup
        let repo = RepoId::new("group/subgroup", "project");
        let url = provider.project_url(&repo, "/merge_requests");
        assert!(url.contains("projects/group%2Fsubgroup%2Fproject/merge_requests"));
    }

    #[test]
    fn test_pipeline_status_parsing() {
        assert_eq!(parse_pipeline_status("pending"), PipelineStatus::Pending);
        assert_eq!(parse_pipeline_status("running"), PipelineStatus::Running);
        assert_eq!(parse_pipeline_status("success"), PipelineStatus::Success);
        assert_eq!(parse_pipeline_status("failed"), PipelineStatus::Failed);
        assert_eq!(parse_pipeline_status("canceled"), PipelineStatus::Canceled);
        assert_eq!(parse_pipeline_status("cancelled"), PipelineStatus::Canceled);
        assert_eq!(parse_pipeline_status("skipped"), PipelineStatus::Skipped);
        assert_eq!(
            parse_pipeline_status("unknown_status"),
            PipelineStatus::Unknown
        );
    }
}
