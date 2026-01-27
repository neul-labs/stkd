# Building from Source

Building Stack from source gives you the latest features and allows customization.

## Prerequisites

You'll need:

- **Rust** 1.70 or later ([rustup.rs](https://rustup.rs))
- **Git** 2.28 or later
- A C compiler (for libgit2)
- OpenSSL development libraries

### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install build-essential libssl-dev pkg-config
```

### Linux (Fedora)

```bash
sudo dnf install gcc openssl-devel
```

### macOS

```bash
xcode-select --install
brew install openssl
```

### Windows

Install Visual Studio Build Tools with the "C++ build tools" workload.

## Installation

### From crates.io (Recommended)

```bash
cargo install stkd-cli
```

### From Git Repository

```bash
# Latest release
cargo install --git https://github.com/neul-labs/stack stkd-cli

# Specific version
cargo install --git https://github.com/neul-labs/stack --tag v0.1.0 stkd-cli

# Development version
cargo install --git https://github.com/neul-labs/stack --branch main stkd-cli
```

### Local Development

```bash
# Clone the repository
git clone https://github.com/neul-labs/stack
cd stack

# Build in debug mode
cargo build

# Run directly
cargo run -- --help

# Build in release mode
cargo build --release

# Install locally
cargo install --path crates/stkd-cli
```

## Feature Flags

Stack supports optional feature flags:

```bash
# Install with all providers
cargo install stkd-cli --features all-providers

# Install with only GitHub support (default)
cargo install stkd-cli --features github

# Install with GitLab support
cargo install stkd-cli --features gitlab
```

## Updating

To update to the latest version:

```bash
cargo install stkd-cli --force
```

## Troubleshooting

### OpenSSL Errors

If you see OpenSSL-related errors:

```bash
# Linux
export OPENSSL_DIR=/usr/lib/ssl

# macOS with Homebrew
export OPENSSL_DIR=$(brew --prefix openssl)
```

### libgit2 Errors

Stack uses libgit2 via the `git2` crate. If you encounter issues:

```bash
# Use bundled libgit2
LIBGIT2_SYS_USE_PKG_CONFIG=0 cargo install stkd-cli
```

### Build Failures

Ensure you have the latest Rust:

```bash
rustup update stable
```
