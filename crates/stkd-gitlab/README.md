# stkd-gitlab

[![crates.io](https://img.shields.io/crates/v/stkd-gitlab)](https://crates.io/crates/stkd-gitlab)
[![docs.rs](https://img.shields.io/badge/docs.rs-stkd--gitlab-blue)](https://docs.rs/stkd-gitlab)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

GitLab integration for [Stack](https://github.com/neul-labs/stkd) — stacked diffs for Git.

This crate implements the Stack provider API for GitLab, including self-hosted instances. It supports authentication via personal access tokens, OAuth, and job tokens, and provides operations for creating and managing merge requests, checking CI/CD pipelines, and handling reviews.

## Installation

```bash
cargo add stkd-gitlab
```

## Usage

```rust
use stkd_gitlab::GitLabProvider;
use stkd_provider_api::{Provider, RepoId, CreateMergeRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For gitlab.com
    let provider = GitLabProvider::new("glpat-xxxxxxxx")?;

    // For self-hosted GitLab
    // let provider = GitLabProvider::with_host("glpat-xxxxxxxx", "gitlab.company.com")?;

    let repo_id = RepoId::from("group/project");

    let mr = provider.create_mr(
        &repo_id,
        CreateMergeRequest {
            title: "Add feature".into(),
            source_branch: "feature/x".into(),
            target_branch: "main".into(),
            ..Default::default()
        },
    ).await?;

    println!("Created MR !{}: {}", mr.number, mr.web_url);
    Ok(())
}
```

## Features

- **Personal Access Token** auth
- **OAuth** and **CI job token** support
- **Self-hosted GitLab** support
- **Merge Request** create, update, merge, close
- **Pipeline** status checks
- **Review** requests and discussions

## License

Apache-2.0. See the [repository](https://github.com/neul-labs/stkd) for details.
