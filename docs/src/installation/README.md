# Installation

Stack can be installed in several ways depending on your operating system and preferences.

## Prerequisites

Before installing Stack, ensure you have:

- **Git** 2.28 or later
- **Rust** 1.70+ (if building from source)

## Quick Install

The fastest way to get started depends on your platform:

```bash
# macOS / Linux (Homebrew)
brew install neul-labs/tap/stkd

# Any platform (Cargo)
cargo install stkd-cli

# Any platform (npm)
npm install -g stkd-cli

# Any platform (pip)
pip install stkd-cli
```

## Installation Methods

Choose the method that works best for you:

| Method | Command | Platforms |
|--------|---------|-----------|
| [Homebrew](./package-managers.md#homebrew) | `brew install neul-labs/tap/stkd` | macOS, Linux |
| [Cargo](./from-source.md) | `cargo install stkd-cli` | Any |
| [npm](./package-managers.md#npm) | `npm install -g stkd-cli` | Any (Node.js) |
| [pip](./package-managers.md#pip) | `pip install stkd-cli` | Any (Python) |
| [Binary Download](./package-managers.md#binary-download) | Download from releases | Linux, macOS, Windows |

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
