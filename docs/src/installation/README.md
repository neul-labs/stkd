# Installation

Stack can be installed in several ways depending on your operating system and preferences.

## Prerequisites

Before installing Stack, ensure you have:

- **Git** 2.28 or later
- **Rust** 1.70+ (if building from source)

## Quick Install

The fastest way to get started is to build from source:

```bash
cargo install --git https://github.com/neul-labs/stkd stkd-cli
```

This installs the `gt` command globally.

## Installation Methods

Choose the method that works best for you:

| Method | Command |
|--------|---------|
| [From Source](./from-source.md) | `cargo install` |
| [Homebrew](./package-managers.md#homebrew) | `brew install stack` |
| [Binary Download](./package-managers.md#binary-download) | Download from releases |

## Verify Installation

After installation, verify Stack is working:

```bash
gt --version
# gt 0.1.0

gt --help
# Stack - Stacked Diffs for Git
# ...
```

## Shell Completions

Stack supports shell completions for Bash, Zsh, Fish, and PowerShell:

```bash
# Bash
gt completions bash > ~/.local/share/bash-completion/completions/gt

# Zsh
gt completions zsh > ~/.zfunc/_gt

# Fish
gt completions fish > ~/.config/fish/completions/gt.fish
```

## Next Steps

Once installed:

1. [Authenticate with your provider](../commands/auth.md)
2. [Create your first stack](../getting-started/first-stack.md)
