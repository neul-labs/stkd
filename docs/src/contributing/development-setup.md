# Development Setup

This guide covers setting up a development environment for Stack.

## Prerequisites

- **Rust** 1.70 or later
- **Git** 2.28 or later
- A C compiler (for libgit2)
- OpenSSL development libraries

### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install build-essential libssl-dev pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Linux (Fedora)

```bash
sudo dnf install gcc openssl-devel
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### macOS

```bash
xcode-select --install
brew install openssl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. Install [Rustup](https://rustup.rs/)

## Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/stack
cd stack

# Build in debug mode
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

## Development Workflow

### Running Locally

```bash
# Run with debug logging
RUST_LOG=debug cargo run -- status

# Run a specific command
cargo run -- create test-branch

# Run with release optimizations
cargo run --release -- submit
```

### Building Documentation

```bash
# Build rustdoc
cargo doc --open

# Build mdBook (requires mdbook installed)
cd docs && mdbook serve
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy --all-targets

# With all warnings
cargo clippy --all-targets -- -D warnings
```

## IDE Setup

### VS Code

Recommended extensions:

- rust-analyzer
- Even Better TOML
- CodeLLDB (for debugging)

Settings (`.vscode/settings.json`):

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all"
}
```

### IntelliJ / CLion

Install the Rust plugin. It provides:

- Code completion
- Navigation
- Refactoring
- Debugging

## Project Structure

```
stack/
├── Cargo.toml           # Workspace definition
├── crates/
│   ├── stack-core/      # Core logic
│   ├── stack-provider-api/  # Provider traits
│   ├── stack-github/    # GitHub provider
│   └── stack-cli/       # CLI application
├── docs/                # mdBook documentation
└── tests/               # Integration tests
```

## Debugging

### With println

```rust
println!("DEBUG: value = {:?}", value);
```

### With dbg!

```rust
let result = dbg!(compute_something());
```

### With a Debugger

VS Code launch configuration (`.vscode/launch.json`):

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug gt",
            "cargo": {
                "args": ["build", "--bin=gt", "--package=stack-cli"]
            },
            "args": ["status"],
            "cwd": "${workspaceFolder}/test-repo"
        }
    ]
}
```

## Testing Changes

### Unit Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_branch_creation

# Run tests for a specific crate
cargo test -p stack-core
```

### Integration Tests

```bash
# Run integration tests (requires test repo)
cargo test --test integration
```

### Manual Testing

Create a test repository:

```bash
mkdir test-repo && cd test-repo
git init
git commit --allow-empty -m "Initial commit"

# Now test your changes
../target/debug/gt create test-branch
```

## Submitting Changes

1. Ensure tests pass: `cargo test`
2. Ensure formatting: `cargo fmt`
3. Ensure no warnings: `cargo clippy`
4. Update documentation if needed
5. Create a pull request
