# stkd-core

[![crates.io](https://img.shields.io/crates/v/stkd-core)](https://crates.io/crates/stkd-core)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--core-blue)](https://docs.rs/stkd-core)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Core library for [Stack](https://github.com/neul-labs/stkd) — stacked diffs for Git.

This crate provides the foundational types and operations for managing stacked branches in Git repositories. It handles repository discovery, branch tracking, dependency management (DAG), rebase automation, metadata persistence, and configuration. It is provider-agnostic and intended to be consumed by the CLI, engine, server, and MCP crates.

## Installation

```bash
cargo add stkd-core
```

## Usage

```rust
use stkd_core::Repository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;
    let current = repo.current_branch()?;
    let graph = repo.load_graph()?;

    println!("Current branch: {:?}", current);
    println!("Tracked branches: {}", graph.all_branches().count());

    Ok(())
}
```

## Key Modules

- **`repository`** — `Repository`: the main entry point for Git + Stack operations
- **`storage`** — `Storage`, `StackState`, `OperationPhase`: persistent metadata in `.git/stkd/`
- **`dag`** — `BranchGraph`: directed acyclic graph of branch parent-child relationships
- **`stack`** — `Stack`, `StackEntry`: linearized view of a branch chain
- **`branch`** — `BranchInfo`: metadata for a single tracked branch
- **`rebase`** — `rebase_branch`, `restack_all`: automated rebase operations with conflict detection
- **`config`** — `StackConfig`, `ProviderConfig`: repository and provider settings
- **`error`** — `Error`: typed errors with recovery hints

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
