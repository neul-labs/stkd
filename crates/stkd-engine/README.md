# stkd-engine

[![crates.io](https://img.shields.io/crates/v/stkd-engine.svg)](https://crates.io/crates/stkd-engine)
[![docs.rs](https://docs.rs/stkd-engine/badge.svg)](https://docs.rs/stkd-engine)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Programmatic engine for Stack** — exposes `gt` operations as reusable library functions for multi-agent harnesses, CI systems, and IDE plugins.

`stkd-engine` provides a programmatic API for stacked diffs, designed for consumption by automated systems. All functions return structured, serializable results (`SubmitResult`, `SyncResult`, `LandResult`, etc.) that can be consumed by AI agents, CI pipelines, and custom tooling.

---

## What is Stack?

Stack is an open-source, **Graphite-compatible** CLI for managing stacked pull requests on GitHub and GitLab. It breaks large changes into small, reviewable PRs that stay in sync automatically. `stkd-engine` lets you integrate Stack operations into your own applications.

## Why Stacked Diffs?

Large PRs are hard to review. Stacked diffs let you split work into a chain of dependent branches, each with its own focused PR. When you update a branch, Stack automatically rebases all dependent branches. When PRs merge, the stack collapses cleanly.

## Installation

```bash
cargo add stkd-engine
```

## Usage

```rust
use stkd_engine::{submit, SubmitOptions, ProviderContext};
use stkd_core::Repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;
    let ctx = ProviderContext::from_repo(&repo).await?;

    let result = submit(
        &repo,
        SubmitOptions { stack: true, ..Default::default() },
        ctx.provider(),
        &ctx.repo_id,
    ).await?;

    println!("Created {} PRs", result.created.len());
    Ok(())
}
```

## Key Modules

- **`init`** — `init()`: initialize Stack in a repository
- **`submit`** — `submit()`: push branches and create/update PRs/MRs
- **`sync`** — `sync()`: fetch, restack, clean merged branches
- **`land`** — `land()`: merge PRs/MRs and clean up
- **`restack`** — `restack()`: rebase branches onto updated parents
- **`retry`** — `with_retry()`: exponential backoff for provider calls
- **`provider`** — `ProviderContext`: auto-detect provider from git remotes

## Features

- **Structured results** — All operations return typed, serializable results
- **Async/await** — Built on Tokio for concurrent provider operations
- **Auto-retry** — Exponential backoff with jitter for API calls
- **Provider auto-detection** — Detects GitHub/GitLab from git remotes
- **Zero-config defaults** — Sensible defaults for all options

## Related Crates

- [`stkd-cli`](https://crates.io/crates/stkd-cli) — The main CLI binary
- [`stkd-core`](https://crates.io/crates/stkd-core) — Core library
- [`stkd-mcp`](https://crates.io/crates/stkd-mcp) — MCP server for AI agents

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
