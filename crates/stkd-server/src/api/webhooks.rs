//! Webhook handlers for GitHub and GitLab events.

use axum::{extract::State, http::HeaderMap, routing::post, Json, Router};
use serde::Deserialize;

use crate::error::ApiResult;
use crate::state::AppState;

/// GitHub webhook payload.
#[derive(Debug, Deserialize)]
pub struct GitHubWebhook {
    action: Option<String>,
    pull_request: Option<GitHubPullRequest>,
    repository: Option<GitHubRepository>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPullRequest {
    number: u64,
    title: String,
    state: String,
    html_url: String,
    head: GitHubRef,
    base: GitHubRef,
    merged: Option<bool>,
    draft: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubRef {
    #[serde(rename = "ref")]
    ref_name: String,
    sha: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields read by serde
pub struct GitHubRepository {
    full_name: String,
    default_branch: String,
}

/// GitLab webhook payload.
#[derive(Debug, Deserialize)]
pub struct GitLabWebhook {
    object_kind: String,
    object_attributes: Option<GitLabMergeRequest>,
    project: Option<GitLabProject>,
}

#[derive(Debug, Deserialize)]
pub struct GitLabMergeRequest {
    iid: u64,
    title: String,
    state: String,
    url: String,
    source_branch: String,
    target_branch: String,
    last_commit: Option<GitLabCommit>,
}

#[derive(Debug, Deserialize)]
pub struct GitLabCommit {
    id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Fields read by serde
pub struct GitLabProject {
    path_with_namespace: String,
    default_branch: String,
}

/// Handle GitHub webhook.
async fn github_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<GitHubWebhook>,
) -> ApiResult<Json<serde_json::Value>> {
    let event = headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    tracing::info!("Received GitHub webhook: {}", event);

    match event {
        "pull_request" => {
            if let (Some(pr), Some(repo)) = (payload.pull_request, payload.repository) {
                handle_github_pr_event(&state, &pr, &repo, payload.action.as_deref()).await?;
            }
        }
        "push" => {
            // Handle push events for branch updates
            tracing::debug!("GitHub push event received");
        }
        _ => {
            tracing::debug!("Ignoring GitHub event: {}", event);
        }
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Handle GitHub pull request events.
async fn handle_github_pr_event(
    state: &AppState,
    pr: &GitHubPullRequest,
    repo: &GitHubRepository,
    action: Option<&str>,
) -> ApiResult<()> {
    tracing::info!(
        "GitHub PR #{} {} on {}: {}",
        pr.number,
        action.unwrap_or("unknown"),
        repo.full_name,
        pr.title
    );

    // Find the repository in our database
    let parts: Vec<&str> = repo.full_name.split('/').collect();
    if parts.len() != 2 {
        return Ok(());
    }

    let db_repo = state
        .db()
        .repositories()
        .get_by_provider("github", parts[0], parts[1])
        .await?;

    let Some(db_repo) = db_repo else {
        tracing::debug!("Repository {} not found in database", repo.full_name);
        return Ok(());
    };

    // Find or create the branch
    let branch = state
        .db()
        .branches()
        .find_or_create(db_repo.id, &pr.head.ref_name, Some(&pr.base.ref_name))
        .await?;

    // Update branch head SHA
    state
        .db()
        .branches()
        .update_head(branch.id, &pr.head.sha)
        .await?;

    // Update or create merge request
    let mr_state = if pr.merged.unwrap_or(false) {
        stkd_db::MergeRequestState::Merged
    } else if pr.draft.unwrap_or(false) {
        stkd_db::MergeRequestState::Draft
    } else if pr.state == "closed" {
        stkd_db::MergeRequestState::Closed
    } else {
        stkd_db::MergeRequestState::Open
    };

    let mr = state
        .db()
        .merge_requests()
        .find_or_create_by_number(
            db_repo.id,
            branch.id,
            pr.number,
            &pr.title,
            &pr.html_url,
            &pr.head.ref_name,
            &pr.base.ref_name,
            &pr.number.to_string(),
        )
        .await?;

    // Update MR state
    state
        .db()
        .merge_requests()
        .update_state(mr.id, mr_state)
        .await?;

    // Update branch status and MR reference
    let branch_status = match mr_state {
        stkd_db::MergeRequestState::Merged => stkd_db::BranchStatus::Merged,
        stkd_db::MergeRequestState::Closed => stkd_db::BranchStatus::Closed,
        _ => stkd_db::BranchStatus::Active,
    };
    state
        .db()
        .branches()
        .update_status(branch.id, branch_status)
        .await?;
    state.db().branches().set_mr(branch.id, Some(mr.id)).await?;

    Ok(())
}

/// Handle GitLab webhook.
async fn gitlab_webhook(
    State(state): State<AppState>,
    _headers: HeaderMap, // TODO: Verify webhook signature
    Json(payload): Json<GitLabWebhook>,
) -> ApiResult<Json<serde_json::Value>> {
    tracing::info!("Received GitLab webhook: {}", payload.object_kind);

    match payload.object_kind.as_str() {
        "merge_request" => {
            if let (Some(mr), Some(project)) = (payload.object_attributes, payload.project) {
                handle_gitlab_mr_event(&state, &mr, &project).await?;
            }
        }
        "push" => {
            tracing::debug!("GitLab push event received");
        }
        _ => {
            tracing::debug!("Ignoring GitLab event: {}", payload.object_kind);
        }
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Handle GitLab merge request events.
async fn handle_gitlab_mr_event(
    state: &AppState,
    mr: &GitLabMergeRequest,
    project: &GitLabProject,
) -> ApiResult<()> {
    tracing::info!(
        "GitLab MR !{} on {}: {}",
        mr.iid,
        project.path_with_namespace,
        mr.title
    );

    // Find the repository in our database
    let parts: Vec<&str> = project.path_with_namespace.split('/').collect();
    if parts.len() < 2 {
        return Ok(());
    }

    let db_repo = state
        .db()
        .repositories()
        .get_by_provider("gitlab", parts[0], parts[parts.len() - 1])
        .await?;

    let Some(db_repo) = db_repo else {
        tracing::debug!(
            "Repository {} not found in database",
            project.path_with_namespace
        );
        return Ok(());
    };

    // Find or create the branch
    let branch = state
        .db()
        .branches()
        .find_or_create(db_repo.id, &mr.source_branch, Some(&mr.target_branch))
        .await?;

    // Update branch head SHA if available
    if let Some(commit) = &mr.last_commit {
        state
            .db()
            .branches()
            .update_head(branch.id, &commit.id)
            .await?;
    }

    // Determine MR state
    let mr_state = match mr.state.as_str() {
        "merged" => stkd_db::MergeRequestState::Merged,
        "closed" => stkd_db::MergeRequestState::Closed,
        _ => stkd_db::MergeRequestState::Open,
    };

    // Find or create merge request
    let db_mr = state
        .db()
        .merge_requests()
        .find_or_create_by_number(
            db_repo.id,
            branch.id,
            mr.iid,
            &mr.title,
            &mr.url,
            &mr.source_branch,
            &mr.target_branch,
            &mr.iid.to_string(),
        )
        .await?;

    // Update MR state
    state
        .db()
        .merge_requests()
        .update_state(db_mr.id, mr_state)
        .await?;

    // Update branch status and MR reference
    let branch_status = match mr_state {
        stkd_db::MergeRequestState::Merged => stkd_db::BranchStatus::Merged,
        stkd_db::MergeRequestState::Closed => stkd_db::BranchStatus::Closed,
        _ => stkd_db::BranchStatus::Active,
    };
    state
        .db()
        .branches()
        .update_status(branch.id, branch_status)
        .await?;
    state
        .db()
        .branches()
        .set_mr(branch.id, Some(db_mr.id))
        .await?;

    Ok(())
}

/// Build webhook routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/github", post(github_webhook))
        .route("/gitlab", post(gitlab_webhook))
}
