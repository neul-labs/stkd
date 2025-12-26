# Implementing a Provider

This guide explains how to add support for a new Git hosting provider to Stack.

## Overview

Stack uses a trait-based provider system defined in the `stack-provider-api` crate. To add a new provider:

1. Create a new crate (e.g., `stack-gitea`)
2. Implement the required traits
3. Register the provider in the CLI

## Required Traits

Every provider must implement these core traits:

```rust
use stack_provider_api::{
    MergeRequestProvider,
    UserProvider,
    RepositoryProvider,
    Provider,
};
```

### MergeRequestProvider

Core merge request operations:

```rust
#[async_trait]
impl MergeRequestProvider for MyProvider {
    async fn create_mr(&self, repo: &RepoId, request: CreateMergeRequest)
        -> ProviderResult<MergeRequest>;

    async fn update_mr(&self, repo: &RepoId, id: MergeRequestId, update: UpdateMergeRequest)
        -> ProviderResult<MergeRequest>;

    async fn get_mr(&self, repo: &RepoId, id: MergeRequestId)
        -> ProviderResult<MergeRequest>;

    async fn list_mrs(&self, repo: &RepoId, filter: MergeRequestFilter)
        -> ProviderResult<Vec<MergeRequest>>;

    async fn merge_mr(&self, repo: &RepoId, id: MergeRequestId, method: MergeMethod)
        -> ProviderResult<MergeResult>;

    async fn close_mr(&self, repo: &RepoId, id: MergeRequestId)
        -> ProviderResult<MergeRequest>;

    async fn reopen_mr(&self, repo: &RepoId, id: MergeRequestId)
        -> ProviderResult<MergeRequest>;
}
```

### UserProvider

User and authentication:

```rust
#[async_trait]
impl UserProvider for MyProvider {
    async fn current_user(&self) -> ProviderResult<User>;
    async fn validate_auth(&self) -> ProviderResult<bool>;
    async fn get_user(&self, username: &str) -> ProviderResult<User>;
}
```

### RepositoryProvider

Repository operations:

```rust
#[async_trait]
impl RepositoryProvider for MyProvider {
    async fn check_access(&self, repo: &RepoId) -> ProviderResult<bool>;
    async fn get_default_branch(&self, repo: &RepoId) -> ProviderResult<String>;
    fn parse_remote_url(&self, url: &str) -> Option<RepoId>;
}
```

### Provider

Tie it all together:

```rust
impl Provider for MyProvider {
    fn name(&self) -> &'static str { "myprovider" }
    fn display_name(&self) -> &'static str { "My Provider" }
    fn capabilities(&self) -> ProviderCapabilities;

    // Optional features
    fn pipelines(&self) -> Option<&dyn PipelineProvider> { None }
    fn approvals(&self) -> Option<&dyn ApprovalProvider> { None }
    fn labels(&self) -> Option<&dyn LabelProvider> { None }
    fn milestones(&self) -> Option<&dyn MilestoneProvider> { None }
}
```

## Optional Traits

Implement these for additional features:

### PipelineProvider

CI/CD status:

```rust
#[async_trait]
impl PipelineProvider for MyProvider {
    async fn get_pipeline_status(&self, repo: &RepoId, ref_name: &str)
        -> ProviderResult<Option<Pipeline>>;
    async fn list_mr_pipelines(&self, repo: &RepoId, mr_id: MergeRequestId)
        -> ProviderResult<Vec<Pipeline>>;
    async fn trigger_pipeline(&self, repo: &RepoId, ref_name: &str)
        -> ProviderResult<Pipeline>;
    async fn cancel_pipeline(&self, repo: &RepoId, pipeline_id: u64)
        -> ProviderResult<()>;
    async fn retry_pipeline(&self, repo: &RepoId, pipeline_id: u64)
        -> ProviderResult<Pipeline>;
}
```

### ApprovalProvider

Code review:

```rust
#[async_trait]
impl ApprovalProvider for MyProvider {
    async fn list_reviews(&self, repo: &RepoId, mr_id: MergeRequestId)
        -> ProviderResult<Vec<Review>>;
    async fn request_review(&self, repo: &RepoId, mr_id: MergeRequestId, reviewers: Vec<String>)
        -> ProviderResult<()>;
    async fn get_approval_status(&self, repo: &RepoId, mr_id: MergeRequestId)
        -> ProviderResult<ApprovalState>;
    async fn has_required_approvals(&self, repo: &RepoId, mr_id: MergeRequestId)
        -> ProviderResult<bool>;
}
```

## Example Implementation

Here's a minimal provider skeleton:

```rust
use async_trait::async_trait;
use stack_provider_api::*;

pub struct GiteaProvider {
    client: reqwest::Client,
    base_url: String,
    token: String,
}

impl GiteaProvider {
    pub fn new(base_url: &str, token: &str) -> ProviderResult<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
            token: token.to_string(),
        })
    }
}

#[async_trait]
impl MergeRequestProvider for GiteaProvider {
    async fn create_mr(&self, repo: &RepoId, request: CreateMergeRequest)
        -> ProviderResult<MergeRequest>
    {
        let url = format!(
            "{}/api/v1/repos/{}/{}/pulls",
            self.base_url, repo.owner, repo.name
        );

        // Make API call and convert response
        todo!()
    }

    // ... implement other methods
}

// ... implement other traits
```

## Testing

Write integration tests that can run against a real instance:

```rust
#[tokio::test]
#[ignore] // Run manually with credentials
async fn test_create_mr() {
    let provider = GiteaProvider::new(
        &std::env::var("GITEA_URL").unwrap(),
        &std::env::var("GITEA_TOKEN").unwrap(),
    ).unwrap();

    let repo = RepoId::new("test-owner", "test-repo");
    let mr = provider.create_mr(&repo, CreateMergeRequest {
        title: "Test PR".to_string(),
        source_branch: "test-branch".to_string(),
        target_branch: "main".to_string(),
        ..Default::default()
    }).await.unwrap();

    assert!(mr.id.0 > 0);
}
```

## Registration

Add your provider to the CLI:

```rust
// In stack-cli/src/provider_context.rs
fn create_provider(config: &ProviderConfig) -> Box<dyn Provider> {
    match config.provider_type {
        ProviderType::GitHub => Box::new(GitHubProvider::new(...)),
        ProviderType::GitLab => Box::new(GitLabProvider::new(...)),
        ProviderType::Gitea => Box::new(GiteaProvider::new(...)),
        // Add your provider here
    }
}
```

## Contributing

When your provider is ready:

1. Add documentation in `docs/src/providers/`
2. Update the provider comparison table
3. Add to feature flags in `Cargo.toml`
4. Submit a pull request

See [Contributing](../contributing/README.md) for guidelines.
