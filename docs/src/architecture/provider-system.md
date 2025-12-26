# Provider System

Stack's provider system enables support for multiple Git hosting platforms through a trait-based architecture.

## Design Goals

1. **Abstraction**: CLI code doesn't know which provider is being used
2. **Extensibility**: New providers can be added without modifying core code
3. **Consistency**: Same operations work the same way across providers
4. **Capability Discovery**: Providers can indicate which features they support

## Trait Hierarchy

```
Provider (main entry point)
│
├── MergeRequestProvider (required)
│   - create_mr, update_mr, get_mr
│   - list_mrs, merge_mr, close_mr
│
├── UserProvider (required)
│   - current_user, validate_auth
│
├── RepositoryProvider (required)
│   - check_access, get_default_branch
│   - parse_remote_url
│
├── PipelineProvider (optional)
│   - get_pipeline_status, list_mr_pipelines
│   - trigger_pipeline, cancel_pipeline
│
├── ApprovalProvider (optional)
│   - list_reviews, request_review
│   - get_approval_status
│
├── LabelProvider (optional)
│   - list_labels, add_labels, remove_labels
│
└── MilestoneProvider (optional)
    - list_milestones, assign_milestone
```

## Capability Discovery

Providers declare what they support:

```rust
impl Provider for GitHubProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            merge_requests: true,
            pipelines: true,        // GitHub Actions
            approvals: true,        // Reviews
            labels: true,
            milestones: true,
            draft_mrs: true,
            squash_merge: true,
            rebase_merge: true,
            fast_forward_merge: false,  // Not supported
        }
    }
}
```

CLI code can check capabilities:

```rust
fn maybe_show_pipeline_status(provider: &dyn Provider, ...) {
    if provider.capabilities().pipelines {
        if let Some(pipeline_provider) = provider.pipelines() {
            // Show pipeline status
        }
    }
}
```

## Type Mapping

Each provider maps its API types to the common types:

```rust
// GitHub PR → MergeRequest
impl From<GhPullRequest> for MergeRequest {
    fn from(pr: GhPullRequest) -> Self {
        MergeRequest {
            id: MergeRequestId(pr.number),
            title: pr.title,
            state: match pr.state.as_str() {
                "open" => MergeRequestState::Open,
                "closed" if pr.merged => MergeRequestState::Merged,
                "closed" => MergeRequestState::Closed,
                _ => MergeRequestState::Open,
            },
            source_branch: pr.head.ref_name,
            target_branch: pr.base.ref_name,
            // ...
        }
    }
}
```

## Error Handling

Providers return `ProviderResult<T>` with standardized errors:

```rust
pub enum ProviderError {
    AuthenticationFailed(String),
    AuthorizationDenied(String),
    NotFound(String),
    RateLimited { retry_after: Option<u64> },
    MergeConflict(String),
    ValidationError(String),
    NetworkError(String),
    UnsupportedOperation(String),
    ProviderSpecific(String),
    Internal(String),
}
```

The CLI handles these uniformly:

```rust
match provider.create_mr(&repo, request).await {
    Ok(mr) => output::success(&format!("Created {}", mr.id)),
    Err(ProviderError::AuthenticationFailed(_)) => {
        output::error("Not authenticated. Run 'gt auth'");
    }
    Err(ProviderError::RateLimited { retry_after }) => {
        output::error("Rate limited. Try again later.");
    }
    Err(e) => output::error(&e.to_string()),
}
```

## Provider Selection

The CLI determines which provider to use:

```rust
fn get_provider(repo: &Repository) -> Box<dyn Provider> {
    let config = repo.config().provider.effective();

    match config.provider_type {
        ProviderType::Auto => auto_detect_provider(repo),
        ProviderType::GitHub => create_github_provider(config),
        ProviderType::GitLab => create_gitlab_provider(config),
        ProviderType::Gitea => create_gitea_provider(config),
    }
}

fn auto_detect_provider(repo: &Repository) -> Box<dyn Provider> {
    let remote_url = repo.get_remote_url("origin");

    // Try each provider's URL parser
    if GitHubProvider::parse_url(&remote_url).is_some() {
        return create_github_provider(...);
    }
    if GitLabProvider::parse_url(&remote_url).is_some() {
        return create_gitlab_provider(...);
    }

    // Default or error
}
```

## Authentication Flow

```
User runs: gt auth login --github

1. CLI determines provider type
2. CLI loads provider-specific auth module
3. Provider initiates OAuth or token flow
4. Credentials stored in CredentialStore
5. Provider validates credentials
6. Success/failure reported to user
```

## Adding a New Provider

1. **Create crate**: `stack-myprovider`
2. **Implement traits**: Required + optional as supported
3. **Add to CLI**: Provider selection logic
4. **Add to build**: Feature flags in Cargo.toml
5. **Document**: Provider-specific docs

See [Implementing a Provider](../providers/implementing.md) for details.
