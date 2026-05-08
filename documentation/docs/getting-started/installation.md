# Installation

Stack can be installed via several package managers or built from source.

## Requirements

- Git 2.23 or later
- Rust 1.70+ (for building from source)

## Install via Homebrew (macOS / Linux)

The easiest way to install Stack on macOS and Linux:

```bash
brew install neul-labs/tap/stkd
```

Update with:

```bash
brew upgrade stkd
```

## Install via Cargo

```bash
cargo install stkd-cli
```

This installs the `gt` command globally.

## Install via npm

```bash
npm install -g stkd-cli
```

## Install via pip

```bash
pip install stkd-cli
```

## Quick Install Script

Download the latest binary or build from source automatically:

```bash
curl -fsSL https://raw.githubusercontent.com/neul-labs/stkd/main/install.sh | bash
```

## Build from Source

For the latest development version:

```bash
# Clone the repository
git clone https://github.com/neul-labs/stkd
cd stkd

# Build and install
cargo install --path crates/stkd-cli
```

## Verify Installation

Check that Stack is installed correctly:

```bash
gt --version
```

You should see output like:

```
gt 0.1.0
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
# Homebrew
brew upgrade stkd

# Cargo
cargo install stkd-cli --force

# npm
npm update -g stkd-cli

# pip
pip install --upgrade stkd-cli
```

## Uninstalling

To remove Stack:

```bash
# Homebrew
brew uninstall stkd

# Cargo
cargo uninstall stkd-cli

# npm
npm uninstall -g stkd-cli

# pip
pip uninstall stkd-cli
```

## Next Steps

- [Quick Start](quickstart.md) - Get started in 5 minutes
- [Authentication](authentication.md) - Connect to GitHub or GitLab
