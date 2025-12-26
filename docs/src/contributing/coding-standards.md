# Coding Standards

This document outlines the coding standards for the Stack project.

## Rust Style

We follow standard Rust conventions:

- **Formatting**: Use `rustfmt` (run `cargo fmt`)
- **Linting**: Use `clippy` (run `cargo clippy`)
- **Naming**: Follow Rust naming conventions

## Code Organization

### Modules

- One concept per module
- Keep files under 500 lines when possible
- Use submodules for complex features

### Imports

```rust
// Standard library first
use std::collections::HashMap;

// External crates second
use anyhow::Result;
use serde::{Deserialize, Serialize};

// Internal crates third
use stack_core::Repository;
```

### Visibility

- Prefer private by default
- Use `pub(crate)` for crate-internal APIs
- Use `pub` only for public API

## Error Handling

### Use anyhow for Applications

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;

    toml::from_str(&content)
        .context("Failed to parse config")
}
```

### Use thiserror for Libraries

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Not found: {0}")]
    NotFound(String),
}
```

## Documentation

### Public Items

All public items need documentation:

```rust
/// Creates a new stacked branch.
///
/// # Arguments
///
/// * `name` - The name of the new branch
/// * `parent` - The parent branch to base on
///
/// # Returns
///
/// The newly created branch info.
///
/// # Errors
///
/// Returns an error if:
/// - The branch already exists
/// - The parent branch doesn't exist
///
/// # Examples
///
/// ```rust
/// let branch = repo.create_branch("feature/new", "main")?;
/// ```
pub fn create_branch(&self, name: &str, parent: &str) -> Result<BranchInfo> {
    // ...
}
```

### Private Items

Document complex logic:

```rust
// Topologically sort branches for restacking.
// Ensures parents are always processed before children.
fn sort_for_restack(branches: &[String]) -> Vec<String> {
    // ...
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_creation() {
        let repo = TestRepo::new();
        let branch = repo.create_branch("test", "main").unwrap();

        assert_eq!(branch.name, "test");
        assert_eq!(branch.parent, "main");
    }
}
```

### Test Naming

Use descriptive names:

```rust
#[test]
fn create_branch_with_existing_name_returns_error() { }

#[test]
fn restack_preserves_commits_after_conflict_resolution() { }
```

## Async Code

### Use async-trait

```rust
use async_trait::async_trait;

#[async_trait]
trait Provider {
    async fn create_mr(&self, ...) -> Result<MergeRequest>;
}
```

### Avoid Blocking

```rust
// Bad: blocking in async context
async fn read_file() -> Result<String> {
    std::fs::read_to_string("file.txt")  // Blocks!
}

// Good: use async file I/O
async fn read_file() -> Result<String> {
    tokio::fs::read_to_string("file.txt").await
}
```

## Performance

### Avoid Unnecessary Clones

```rust
// Bad
fn process(data: &String) -> String {
    data.clone()
}

// Good
fn process(data: &str) -> &str {
    data
}
```

### Use Iterators

```rust
// Bad
let mut results = Vec::new();
for item in items {
    results.push(transform(item));
}

// Good
let results: Vec<_> = items.iter().map(transform).collect();
```

## Security

### Never Log Secrets

```rust
// Bad
tracing::debug!("Token: {}", token);

// Good
tracing::debug!("Token: [redacted]");
```

### Validate Input

```rust
fn set_branch_name(name: &str) -> Result<()> {
    if name.contains("..") {
        anyhow::bail!("Invalid branch name");
    }
    // ...
}
```

## Commit Messages

Format:

```
<type>: <short summary>

<optional body>

<optional footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `refactor`: Code refactoring
- `test`: Tests
- `chore`: Build, CI, etc.

Examples:

```
feat: Add GitLab provider support

Implements the Provider traits for GitLab,
including MR creation and pipeline status.

Closes #123
```

```
fix: Handle null response in PR API

The GitHub API can return null for mergeable
status before checks complete.

Fixes #456
```
