# stkd-github — GitHub Pull Request Automation for Stack

[![crates.io](https://img.shields.io/crates/v/stkd-github.svg)](https://crates.io/crates/stkd-github)
[![docs.rs](https://docs.rs/stkd-github/badge.svg)](https://docs.rs/stkd-github)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**GitHub integration for Stack** — full support for GitHub pull requests, reviews, CI status, and OAuth authentication.

`stkd-github` implements the Stack provider API for GitHub. It supports authentication via personal access tokens and OAuth device flow, and provides operations for creating and managing pull requests, checking CI status, and handling reviews.

---

## What is Stack?

Stack is an open-source, **Graphite-compatible** CLI for managing stacked pull requests on GitHub and GitLab. `stkd-github` provides the GitHub-specific implementation that enables Stack to create, update, and merge pull requests.

## Features

- **Personal Access Token** auth
- **OAuth device flow** for CLI applications
- **Pull Request** create, update, merge, close
- **CI status** checks
- **Review** requests and comments
- **Labels** and **milestones**
- **Draft PRs** support

## Installation

```bash
cargo add stkd-github
```

## Usage

```rust
use stkd_github::GitHubProvider;
use stkd_provider_api::{Provider, RepoId, CreateMergeRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = GitHubProvider::new("ghp_xxxxxxxx")?;
    let repo_id = RepoId::from("neul-labs/stkd");

    let mr = provider.create_mr(
        &repo_id,
        CreateMergeRequest {
            title: "Add feature".into(),
            source_branch: "feature/x".into(),
            target_branch: "main".into(),
            ..Default::default()
        },
    ).await?;

    println!("Created PR #{}: {}", mr.number, mr.web_url);
    Ok(())
}
```

## Related Crates

- [`stkd-provider-api`](https://crates.io/crates/stkd-provider-api) — Provider trait definitions
- [`stkd-core`](https://crates.io/crates/stkd-core) — Core library
- [`stkd-cli`](https://crates.io/crates/stkd-cli) — The main CLI binary

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
