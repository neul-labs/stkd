# Testing

This guide covers testing practices for Stack.

## Test Categories

### Unit Tests

Test individual functions and methods:

```bash
cargo test -p stack-core
```

Located in `src/` files with `#[cfg(test)]` modules.

### Integration Tests

Test crate interactions:

```bash
cargo test --test integration
```

Located in `tests/` directories.

### End-to-End Tests

Test the full CLI:

```bash
cargo test --test e2e
```

## Writing Unit Tests

### Basic Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = setup();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

### Testing Errors

```rust
#[test]
fn returns_error_for_invalid_input() {
    let result = parse_branch_name("invalid..name");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}
```

### Testing Async

```rust
#[tokio::test]
async fn async_operation_succeeds() {
    let client = MockClient::new();
    let result = client.fetch_data().await;
    assert!(result.is_ok());
}
```

## Test Fixtures

### Temporary Repositories

```rust
use tempfile::TempDir;

fn create_test_repo() -> (TempDir, Repository) {
    let dir = TempDir::new().unwrap();
    let repo = git2::Repository::init(dir.path()).unwrap();

    // Create initial commit
    let sig = repo.signature().unwrap();
    let tree_id = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[]).unwrap();

    let stack_repo = Repository::open(dir.path()).unwrap();
    (dir, stack_repo)
}
```

### Mock Providers

```rust
struct MockProvider {
    mrs: RefCell<Vec<MergeRequest>>,
}

#[async_trait]
impl MergeRequestProvider for MockProvider {
    async fn create_mr(&self, _, request: CreateMergeRequest)
        -> ProviderResult<MergeRequest>
    {
        let mr = MergeRequest {
            id: MergeRequestId(1),
            title: request.title,
            // ...
        };
        self.mrs.borrow_mut().push(mr.clone());
        Ok(mr)
    }
}
```

## Integration Tests

### Setup

```rust
// tests/common/mod.rs
pub struct TestContext {
    pub dir: TempDir,
    pub repo: Repository,
}

impl TestContext {
    pub fn new() -> Self {
        let dir = TempDir::new().unwrap();
        // ... setup
        Self { dir, repo }
    }
}
```

### Example Test

```rust
// tests/integration/stack_operations.rs
use common::TestContext;

#[test]
fn create_and_submit_stack() {
    let ctx = TestContext::new();

    // Create first branch
    ctx.repo.create_branch("feature/a").unwrap();
    ctx.repo.commit("First change").unwrap();

    // Stack second branch
    ctx.repo.create_branch("feature/b").unwrap();
    ctx.repo.commit("Second change").unwrap();

    // Verify stack structure
    let graph = ctx.repo.load_graph().unwrap();
    assert_eq!(graph.stack("feature/b"), vec!["feature/a", "feature/b"]);
}
```

## Mocking HTTP

For provider tests:

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn creates_pr_successfully() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/repos/owner/repo/pulls"))
        .respond_with(ResponseTemplate::new(201)
            .set_body_json(json!({
                "number": 42,
                "html_url": "https://github.com/owner/repo/pull/42"
            })))
        .mount(&mock_server)
        .await;

    let provider = GitHubProvider::with_base_url(
        auth,
        &mock_server.uri()
    ).unwrap();

    let result = provider.create_mr(&repo, request).await;
    assert!(result.is_ok());
}
```

## Test Coverage

### Generating Reports

```bash
# Install grcov
cargo install grcov

# Run with coverage
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch -o ./coverage/
```

### Viewing Coverage

Open `coverage/index.html` in a browser.

## CI Testing

Tests run automatically on:

- Every push
- Every pull request

```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
```

## Test Best Practices

1. **Test behavior, not implementation**
2. **Use descriptive test names**
3. **Keep tests focused** - one assertion per test when possible
4. **Don't test private functions** - test through public API
5. **Use fixtures** - avoid duplicating setup code
6. **Clean up** - use `Drop` or `defer` for cleanup
7. **Test edge cases** - empty inputs, errors, boundaries
