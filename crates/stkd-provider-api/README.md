# stkd-provider-api

[![crates.io](https://img.shields.io/crates/v/stkd-provider-api)](https://crates.io/crates/stkd-provider-api)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--provider--api-blue)](https://docs.rs/stkd-provider-api)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Provider API traits and types for [Stack](https://github.com/neul-labs/stkd) — enables pluggable Git hosting provider support.

This crate defines the abstraction layer used by Stack to interact with Git hosting platforms (GitHub, GitLab, and others). It includes traits for merge requests, pipelines, reviews, labels, milestones, and authentication, along with shared types such as `RepoId`, `MergeRequest`, and `ProviderCapabilities`.

## Installation

```bash
cargo add stkd-provider-api
```

## Usage

```rust
use stkd_provider_api::{Provider, RepoId, CreateMergeRequest};

async fn create_mr(provider: &dyn Provider, repo: &RepoId) {
    let request = CreateMergeRequest {
        title: "Add feature X".into(),
        source_branch: "feature/x".into(),
        target_branch: "main".into(),
        draft: false,
        ..Default::default()
    };

    let mr = provider.create_mr(repo, request).await.unwrap();
    println!("Created MR #{}: {}", mr.number, mr.web_url);
}
```

## Key Modules

- **`traits`** — `Provider`, `MergeRequestProvider`, `PipelineProvider`, `ReviewProvider`, `AuthProvider`
- **`types`** — `MergeRequest`, `RepoId`, `MergeRequestState`, `MergeMethod`, `ProviderCapabilities`
- **`error`** — `ProviderError` with retryability classification

## Implementing a Custom Provider

Implement the [`Provider`](https://docs.rs/stkd-provider-api/latest/stkd_provider_api/trait.Provider.html) trait to add support for a new Git hosting platform. See the [provider implementation guide](https://docs.neullabs.com/stkd/providers/custom) for details.

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
