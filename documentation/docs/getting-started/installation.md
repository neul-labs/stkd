# Installation

Stack can be installed via Cargo or built from source.

## Requirements

- Git 2.23 or later
- Rust 1.70+ (for building from source)

## Install via Cargo

The easiest way to install Stack:

```bash
cargo install stack-cli
```

This installs the `gt` command globally.

## Build from Source

For the latest development version:

```bash
# Clone the repository
git clone https://github.com/dipankar/stack
cd stack

# Build and install
cargo install --path crates/stack-cli
```

## Verify Installation

Check that Stack is installed correctly:

```bash
gt --version
```

You should see output like:

```
gt 0.6.0
```

## Shell Completions

Stack supports shell completions for bash, zsh, and fish.

=== "Bash"

    ```bash
    # Add to ~/.bashrc
    eval "$(gt completions bash)"
    ```

=== "Zsh"

    ```bash
    # Add to ~/.zshrc
    eval "$(gt completions zsh)"
    ```

=== "Fish"

    ```bash
    # Add to ~/.config/fish/config.fish
    gt completions fish | source
    ```

## Updating

To update to the latest version:

```bash
cargo install stack-cli --force
```

## Uninstalling

To remove Stack:

```bash
cargo uninstall stack-cli
```

## Next Steps

- [Quick Start](quickstart.md) - Get started in 5 minutes
- [Authentication](authentication.md) - Connect to GitHub or GitLab
