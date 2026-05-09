# stkd-core

[![crates.io](https://img.shields.io/crates/v/stkd-core.svg)](https://crates.io/crates/stkd-core)
[![docs.rs](https://docs.rs/stkd-core/badge.svg)](https://docs.rs/stkd-core)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Core library for Stack** — the foundational types and operations for managing stacked branches in Git repositories.

`stkd-core` is the engine behind [Stack](https://github.com/neul-labs/stkd), an open-source CLI for managing stacked pull requests on GitHub and GitLab. It handles repository discovery, branch tracking, dependency management (DAG), rebase automation, metadata persistence, and configuration. It is provider-agnostic and intended to be consumed by the CLI, engine, server, and MCP crates.

---

## What is Stack?

Stack is a **Graphite-compatible** CLI that breaks large changes into small, reviewable PRs that stay in sync automatically. `stkd-core` provides the underlying Git operations and data structures that make this possible.

## Why Stacked Diffs?

Large PRs are hard to review. Stacked diffs let you split work into a chain of dependent branches, each with its own focused PR. When you update a branch, Stack automatically rebases all dependent branches. When PRs merge, the stack collapses cleanly.

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

## Features

- **Git-native** — Built on `git2` for fast, reliable Git operations
- **Metadata persistence** — Stores stack state in `.git/stkd/` (JSON + Git refs)
- **DAG management** — Tracks branch dependencies as a directed acyclic graph
- **Rebase automation** — Rebases entire stacks with conflict detection and resolution
- **Provider-agnostic** — No hard dependency on GitHub or GitLab
- **Zero-config defaults** — Works out of the box for most repositories

## Related Crates

- [`stkd-cli`](https://crates.io/crates/stkd-cli) — The main CLI binary (`gt`)
- [`stkd-engine`](https://crates.io/crates/stkd-engine) — Programmatic API for CI and agents
- [`stkd-provider-api`](https://crates.io/crates/stkd-provider-api) — Provider trait definitions
- [`stkd-github`](https://crates.io/crates/stkd-github) — GitHub provider implementation
- [`stkd-gitlab`](https://crates.io/crates/stkd-gitlab) — GitLab provider implementation

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
