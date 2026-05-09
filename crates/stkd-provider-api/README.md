# stkd-provider-api

[![crates.io](https://img.shields.io/crates/v/stkd-provider-api.svg)](https://crates.io/crates/stkd-provider-api)
[![docs.rs](https://docs.rs/stkd-provider-api/badge.svg)](https://docs.rs/stkd-provider-api)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Provider API traits and types for Stack** — enables pluggable Git hosting provider support for GitHub, GitLab, and custom platforms.

`stkd-provider-api` defines the abstraction layer used by Stack to interact with Git hosting platforms. It includes traits for merge requests, pipelines, reviews, labels, milestones, and authentication, along with shared types such as `RepoId`, `MergeRequest`, and `ProviderCapabilities`.

---

## What is Stack?

Stack is an open-source, **Graphite-compatible** CLI for managing stacked pull requests on GitHub and GitLab. `stkd-provider-api` makes it possible to add support for new Git hosting platforms without modifying the core library.

## Features

- **Trait-based** — Clean, async traits for all provider operations
- **Shared types** — Common types for repos, PRs/MRs, reviews, pipelines
- **Retry support** — Built-in retryability classification for errors
- **Extensible** — Easy to implement for new platforms

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

## Related Crates

- [`stkd-github`](https://crates.io/crates/stkd-github) — GitHub provider implementation
- [`stkd-gitlab`](https://crates.io/crates/stkd-gitlab) — GitLab provider implementation
- [`stkd-core`](https://crates.io/crates/stkd-core) — Core library

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
