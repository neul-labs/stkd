//! Stack MCP Server
//!
//! Exposes gt operations as MCP tools via stdio transport.
//!
//! # Usage
//!
//! ```json
//! {
//!   "mcpServers": {
//!     "stkd": {
//!       "command": "stkd-mcp"
//!     }
//!   }
//! }
//! ```

use anyhow::Result;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

// ---------------------------------------------------------------------------
// Request structs for tool parameters
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct InitRequest {
    /// Path to the git repository (default: current directory)
    pub path: Option<String>,
    /// Name of the trunk branch (auto-detected if not specified)
    pub trunk: Option<String>,
    /// Name of the git remote (default: origin)
    pub remote: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CreateRequest {
    /// Name for the new branch
    pub name: String,
    /// Create branch from trunk instead of current branch
    pub from_trunk: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LogRequest {
    /// Show all tracked branches, not just the current stack
    pub all: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct StatusRequest {
    /// Fetch latest status from the provider
    pub fetch: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SubmitRequest {
    /// Submit the entire stack (current + descendants)
    pub stack: Option<bool>,
    /// Create MRs as draft
    pub draft: Option<bool>,
    /// Request reviewers (comma-separated usernames)
    pub reviewers: Option<String>,
    /// Add labels (comma-separated)
    pub labels: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SyncRequest {
    /// Don't delete merged branches
    pub no_delete: Option<bool>,
    /// Don't restack after sync
    pub no_restack: Option<bool>,
    /// Don't update trunk
    pub no_pull: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LandRequest {
    /// Merge method: merge, squash, rebase, or ff
    pub method: Option<String>,
    /// Land the entire stack from bottom to top
    pub stack: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RestackRequest {
    /// Only restack current branch and descendants
    pub current_only: Option<bool>,
    /// Force restack even if branches appear up-to-date
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct TrackRequest {
    /// Name of the branch to track
    pub branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DeleteRequest {
    /// Name of the branch to delete
    pub branch: String,
    /// Force delete even if branch has unmerged changes
    pub force: Option<bool>,
}

// ---------------------------------------------------------------------------
// Server
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct StkdMcpServer {
    tool_router: ToolRouter<Self>,
}

impl StkdMcpServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for StkdMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for StkdMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Stack (stkd) MCP server: manage stacked branches and merge requests.")
    }
}

fn to_mcp_error<E: std::fmt::Display>(e: E) -> rmcp::ErrorData {
    rmcp::ErrorData::internal_error(format!("{e}"), None)
}

fn open_repo_locked() -> Result<(stkd_core::Repository, stkd_core::RepoLock), rmcp::ErrorData> {
    let repo = stkd_core::Repository::open(".").map_err(to_mcp_error)?;
    let lock = repo.storage().acquire_lock().map_err(to_mcp_error)?;
    Ok((repo, lock))
}

#[tool_router(router = tool_router)]
impl StkdMcpServer {
    #[tool(description = "Initialize Stack in the current git repository")]
    fn gt_init(&self, Parameters(req): Parameters<InitRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let path = req.path.as_deref().unwrap_or(".");
        let opts = stkd_engine::InitOptions {
            trunk: req.trunk,
            remote: req.remote,
            draft_default: false,
            delete_merged: true,
        };

        let result = stkd_engine::init(path, opts).map_err(to_mcp_error)?;
        let content = Content::json(result).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Create a new stacked branch on top of the current branch")]
    fn gt_create(&self, Parameters(req): Parameters<CreateRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;
        repo.ensure_clean().map_err(to_mcp_error)?;

        if req.from_trunk.unwrap_or(false) {
            repo.checkout(repo.trunk()).map_err(to_mcp_error)?;
        }

        let info = repo.create_branch(&req.name).map_err(to_mcp_error)?;
        let content = Content::json(json!({
            "branch": info.name,
            "parent": info.parent,
        })).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Show the current stack of branches")]
    fn gt_log(&self, Parameters(_req): Parameters<LogRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;
        let current = repo.current_branch().map_err(to_mcp_error)?;
        let graph = repo.load_graph().map_err(to_mcp_error)?;

        let center = match current {
            Some(ref c) if repo.storage().is_tracked(c) => c.clone(),
            _ => {
                return Ok(CallToolResult::success(vec![Content::text("Not on a tracked branch")]));
            }
        };

        let stack = stkd_core::Stack::from_graph(&graph, &center, Some(&center));
        let branches: Vec<_> = stack.iter().map(|entry| {
            let branch_info = repo.storage().load_branch(entry.name()).ok().flatten();
            json!({
                "name": entry.name(),
                "is_current": entry.is_current(),
                "mr_number": branch_info.as_ref().and_then(|i| i.merge_request_id),
                "mr_url": branch_info.as_ref().and_then(|i| i.merge_request_url.clone()),
            })
        }).collect();

        let content = Content::json(json!({ "branches": branches })).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Show current branch and merge request status")]
    fn gt_status(&self, Parameters(req): Parameters<StatusRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;
        let current = repo.current_branch().map_err(to_mcp_error)?.unwrap_or_default();
        let tracked = repo.storage().is_tracked(&current);

        let info = if tracked {
            repo.storage().load_branch(&current).map_err(to_mcp_error)?
        } else {
            None
        };

        let mut mr_state = None;
        if let Some(ref info) = info {
            if let Some(mr_number) = info.merge_request_id {
                if req.fetch.unwrap_or(false) {
                    let result = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(async {
                            let ctx = stkd_engine::ProviderContext::from_repo(&repo).await.map_err(to_mcp_error)?;
                            let mr = ctx.provider().get_mr(&ctx.repo_id, stkd_provider_api::MergeRequestId::from(mr_number)).await.map_err(to_mcp_error)?;
                            Ok::<_, rmcp::ErrorData>(mr)
                        })
                    });
                    if let Ok(mr) = result {
                        mr_state = Some(json!({
                            "number": mr_number,
                            "state": format!("{:?}", mr.state),
                            "mergeable": mr.mergeable,
                            "labels": mr.labels,
                        }));
                    }
                } else {
                    mr_state = Some(json!({
                        "number": mr_number,
                        "url": info.merge_request_url,
                    }));
                }
            }
        }

        let content = Content::json(json!({
            "current_branch": current,
            "tracked": tracked,
            "parent": info.as_ref().map(|i| i.parent.clone()),
            "mr": mr_state,
        })).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Submit PRs/MRs for the current branch or stack")]
    fn gt_submit(&self, Parameters(req): Parameters<SubmitRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;

        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let ctx = stkd_engine::ProviderContext::from_repo(&repo).await.map_err(to_mcp_error)?;
                let opts = stkd_engine::SubmitOptions {
                    stack: req.stack.unwrap_or(false),
                    draft: req.draft.unwrap_or(false),
                    reviewers: req.reviewers.map(|s| s.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default(),
                    labels: req.labels.map(|s| s.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default(),
                    ..Default::default()
                };

                let result = stkd_engine::submit(&repo, opts, ctx.provider(), &ctx.repo_id).await.map_err(to_mcp_error)?;
                Ok::<_, rmcp::ErrorData>(result)
            })
        })?;
        let content = Content::json(result).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Sync with remote: fetch, update trunk, check MR status, delete merged branches, and restack")]
    fn gt_sync(&self, Parameters(req): Parameters<SyncRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;

        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let opts = stkd_engine::SyncOptions {
                    no_delete: req.no_delete.unwrap_or(false),
                    no_restack: req.no_restack.unwrap_or(false),
                    no_pull: req.no_pull.unwrap_or(false),
                    ..Default::default()
                };

                let provider = stkd_engine::ProviderContext::from_repo(&repo).await.ok();
                let (provider_ref, repo_id_ref) = match &provider {
                    Some(ctx) => (Some(ctx.provider()), Some(&ctx.repo_id)),
                    None => (None, None),
                };

                let result = stkd_engine::sync(&repo, opts, provider_ref, repo_id_ref).await.map_err(to_mcp_error)?;
                Ok::<_, rmcp::ErrorData>(result)
            })
        })?;
        let content = Content::json(result).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Land (merge) the current branch or stack")]
    fn gt_land(&self, Parameters(req): Parameters<LandRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;

        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let ctx = stkd_engine::ProviderContext::from_repo(&repo).await.map_err(to_mcp_error)?;
                let opts = stkd_engine::LandOptions {
                    method: req.method.unwrap_or_else(|| "squash".to_string()),
                    stack: req.stack.unwrap_or(false),
                    ..Default::default()
                };

                let result = stkd_engine::land(&repo, opts, ctx.provider(), &ctx.repo_id).await.map_err(to_mcp_error)?;
                Ok::<_, rmcp::ErrorData>(result)
            })
        })?;
        let content = Content::json(result).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Restack branches onto their updated parent branches")]
    fn gt_restack(&self, Parameters(req): Parameters<RestackRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;

        let opts = stkd_engine::RestackOptions {
            current_only: req.current_only.unwrap_or(false),
            force: req.force.unwrap_or(false),
            ..Default::default()
        };

        let result = stkd_engine::restack(&repo, opts).map_err(to_mcp_error)?;
        let content = Content::json(result).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Track an existing branch for stacking")]
    fn gt_track(&self, Parameters(req): Parameters<TrackRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;
        repo.track_branch(&req.branch).map_err(to_mcp_error)?;
        let content = Content::json(json!({"tracked": req.branch})).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }

    #[tool(description = "Delete a tracked branch")]
    fn gt_delete(&self, Parameters(req): Parameters<DeleteRequest>) -> Result<CallToolResult, rmcp::ErrorData> {
        let (repo, _lock) = open_repo_locked()?;
        repo.delete_branch(&req.branch, req.force.unwrap_or(false)).map_err(to_mcp_error)?;
        let content = Content::json(json!({"deleted": req.branch})).map_err(to_mcp_error)?;
        Ok(CallToolResult::success(vec![content]))
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .without_time()
        .init();

    let server = StkdMcpServer::new();
    server.serve(rmcp::transport::stdio()).await?.waiting().await?;
    Ok(())
}
