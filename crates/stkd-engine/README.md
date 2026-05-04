# stkd-engine

[![crates.io](https://img.shields.io/crates/v/stkd-engine)](https://crates.io/crates/stkd-engine)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--engine-blue)](https://docs.rs/stkd-engine)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Programmatic engine for [Stack](https://github.com/neul-labs/stkd) — exposes `gt` operations as reusable library functions.

This crate provides a programmatic API for stacked diffs, designed for consumption by multi-agent harnesses, IDE plugins, CI systems, and other programmatic callers. All functions return structured, serializable results (`SubmitResult`, `SyncResult`, `LandResult`, etc.).

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

    println!("Created {} MRs", result.created.len());
    Ok(())
}
```

## Key Modules

- **`init`** — `init()`: initialize Stack in a repository
- **`submit`** — `submit()`: push branches and create/update MRs
- **`sync`** — `sync()`: fetch, restack, clean merged branches
- **`land`** — `land()`: merge MRs and clean up
- **`restack`** — `restack()`: rebase branches onto updated parents
- **`retry`** — `with_retry()`: exponential backoff for provider calls
- **`provider`** — `ProviderContext`: auto-detect provider from git remotes

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
