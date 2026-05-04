# stkd-github

[![crates.io](https://img.shields.io/crates/v/stkd-github)](https://crates.io/crates/stkd-github)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--github-blue)](https://docs.rs/stkd-github)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

GitHub integration for [Stack](https://github.com/neul-labs/stkd) — stacked diffs for Git.

This crate implements the Stack provider API for GitHub. It supports authentication via personal access tokens and OAuth device flow, and provides operations for creating and managing pull requests, checking CI status, and handling reviews.

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

## Features

- **Personal Access Token** auth
- **OAuth device flow** for CLI applications
- **Pull Request** create, update, merge, close
- **CI status** checks
- **Review** requests and comments
- **Labels** and **milestones**

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
