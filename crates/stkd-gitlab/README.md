# stkd-gitlab — GitLab Merge Request Automation for Stack

[![crates.io](https://img.shields.io/crates/v/stkd-gitlab.svg)](https://crates.io/crates/stkd-gitlab)
[![docs.rs](https://docs.rs/stkd-gitlab/badge.svg)](https://docs.rs/stkd-gitlab)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**GitLab integration for Stack** — full support for GitLab merge requests, pipelines, reviews, and self-hosted instances.

`stkd-gitlab` implements the Stack provider API for GitLab, including self-hosted instances. It supports authentication via personal access tokens, OAuth, and job tokens, and provides operations for creating and managing merge requests, checking CI/CD pipelines, and handling reviews.

---

## What is Stack?

Stack is an open-source, **Graphite-compatible** CLI for managing stacked pull requests on GitHub and GitLab. `stkd-gitlab` provides the GitLab-specific implementation that enables Stack to create, update, and merge merge requests — including on self-hosted GitLab instances.

## Features

- **Personal Access Token** auth
- **OAuth** and **CI job token** support
- **Self-hosted GitLab** support
- **Merge Request** create, update, merge, close
- **Pipeline** status checks
- **Review** requests and discussions

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

## Related Crates

- [`stkd-provider-api`](https://crates.io/crates/stkd-provider-api) — Provider trait definitions
- [`stkd-core`](https://crates.io/crates/stkd-core) — Core library
- [`stkd-cli`](https://crates.io/crates/stkd-cli) — The main CLI binary

## License

Apache-2.0. See [LICENSE](https://github.com/neul-labs/stkd/blob/main/LICENSE) for details.
