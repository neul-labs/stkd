# Stack - Stacked Diffs for Git

A Graphite-compatible CLI for managing stacked pull requests on GitHub and GitLab.

## Overview

Stack is an open-source alternative to [Graphite](https://graphite.dev) that brings the stacked diffs workflow to any Git repository. It helps you:

- **Break large changes into reviewable PRs** - Each logical change gets its own branch and PR
- **Keep dependent changes in sync** - When you update a base branch, Stack automatically rebases all dependent branches
- **Submit entire stacks at once** - Create/update PRs for your entire stack with one command
- **Works with GitHub and GitLab** - Full support for both platforms with a unified interface

## Installation

```bash
# Build from source
cargo install --path crates/stack-cli

# The binary is named 'gt' for Graphite compatibility
gt --help

# Enable shell completions (bash, zsh, fish, powershell)
gt completions bash >> ~/.bashrc
gt completions zsh >> ~/.zshrc
gt completions fish > ~/.config/fish/completions/gt.fish
```

## Quick Start

```bash
# Initialize Stack in your repository
cd your-repo
gt init

# Authenticate with your provider
gt auth login              # Interactive OAuth (GitHub/GitLab)
gt auth --token <token>    # Or use a personal access token

# Create a stack of changes
gt create feature/auth-base    # Create first branch
# ... make changes, commit ...

gt create feature/auth-oauth   # Stack another branch on top
# ... make changes, commit ...

gt create feature/auth-tests   # And another
# ... make changes, commit ...

# View your stack
gt log
# ○ feature/auth-base
#   ○ feature/auth-oauth
#     ◉ feature/auth-tests  (you are here)

# Submit all PRs with reviewers and labels
gt submit --stack --reviewers alice,bob --labels feature,auth

# Navigate the stack
gt down          # Go to parent branch
gt up            # Go to child branch
gt top           # Go to stack tip
gt bottom        # Go to stack root

# Update a branch and restack
gt checkout feature/auth-base
# ... make changes ...
gt modify        # Amend the commit
gt restack       # Rebase all dependent branches
```

## Commands

### Branch Management
- `gt create <name>` - Create a new branch on top of current
- `gt rename <name>` - Rename the current branch
- `gt delete <name>` - Delete a branch
- `gt track [branch]` - Start tracking an existing branch
- `gt untrack [branch]` - Stop tracking a branch

### Navigation
- `gt up [n]` - Move up n branches (toward tip)
- `gt down [n]` - Move down n branches (toward root)
- `gt top` - Jump to stack tip
- `gt bottom` - Jump to stack root
- `gt checkout [branch]` - Switch branches (with fuzzy search)

### Stack Operations
- `gt log` - Show the current stack
- `gt ls` - Short stack view
- `gt ll` - Long stack view with details
- `gt info` - Show current branch info
- `gt status` - Show stack status

### Editing
- `gt modify [-m msg]` - Amend the current branch
- `gt squash [--all] [-n count]` - Squash commits in current branch
- `gt fold [--into commit]` - Fold staged changes into a previous commit
- `gt split [-c count]` - Split the current commit into multiple commits

### Synchronization
- `gt sync` - Sync with remote and restack
- `gt restack` - Rebase stack onto updated parents
- `gt submit [--stack]` - Create/update PRs
- `gt land [--stack]` - Merge the stack

### Conflict Resolution
- `gt continue` - Continue after resolving conflicts
- `gt abort` - Abort the current operation

### Configuration
- `gt auth login` - Authenticate with GitHub/GitLab via OAuth
- `gt auth --token <token>` - Authenticate with a personal access token
- `gt config [key] [value]` - View/edit configuration
- `gt completions <shell>` - Generate shell completions

## Advanced Features

### PR Automation

Submit PRs with reviewers, labels, and templates:

```bash
# Request reviewers and add labels
gt submit --reviewers alice,bob --labels bug,urgent

# Use PR template from .github/PULL_REQUEST_TEMPLATE.md
gt submit --template

# Preview what would be done
gt submit --dry-run

# Submit specific branches only
gt submit --only feature/step-1,feature/step-2
gt submit --from feature/step-2  # From branch to tip
gt submit --to feature/step-3    # From root to branch
```

### Commit Operations

```bash
# Squash all commits in branch
gt squash --all

# Squash last 3 commits with custom message
gt squash -n 3 -m "Combined changes"

# Fold staged changes into HEAD
git add file.rs
gt fold

# Fold into a specific commit (creates fixup)
gt fold --into HEAD~2 --fixup

# Split current commit into 3 commits
gt split -c 3
```

### Dry-Run Mode

Preview operations before executing:

```bash
gt submit --dry-run
gt land --dry-run
```

## How It Works

Stack tracks branch relationships in `.git/stack/`:

```
.git/stack/
├── config.json      # Stack configuration
├── state.json       # Current operation state
└── branches/        # Per-branch metadata
    ├── feature__auth-base.json
    ├── feature__auth-oauth.json
    └── feature__auth-tests.json
```

Each branch knows its:
- **Parent** - The branch it was created from
- **Children** - Branches created on top of it
- **PR** - Associated pull request (if submitted)

When you modify a branch, Stack automatically rebases all dependent branches to keep your stack consistent.

## Web Dashboard

Stack includes an optional web dashboard for visualizing and managing your stacks across repositories.

```bash
# Start the dashboard server
cargo run --bin stack-server

# Access at http://localhost:3000
```

Features:
- OAuth login with GitHub/GitLab
- Real-time stack visualization
- Organization-based multi-tenancy
- PR status and CI integration

## Comparison with Graphite

| Feature | Graphite | Stack |
|---------|----------|-------|
| Open source | No | Yes |
| Self-hosted | No | Yes |
| CLI commands | `gt` | `gt` (compatible) |
| GitHub support | Yes | Yes |
| GitLab support | No | Yes |
| Web dashboard | Yes | Yes |
| PR automation | Yes | Yes |
| Shell completions | Yes | Yes |
| AI PR descriptions | Yes (paid) | Planned |

## Project Structure

```
crates/
├── stack-cli          # CLI application (gt binary)
├── stack-core         # Core library (Repository, Stack, DAG)
├── stack-provider-api # Provider trait definitions
├── stack-github       # GitHub implementation
├── stack-gitlab       # GitLab implementation
├── stack-db           # Database abstraction (SQLite/PostgreSQL)
└── stack-server       # Web dashboard API server

web/                   # Vue 3 + TailwindCSS frontend
```

## Development

```bash
# Clone the repository
git clone https://github.com/dipankar/stack
cd stack

# Build all crates
cargo build

# Run tests
cargo test

# Run the CLI
cargo run --bin gt -- --help

# Run the web dashboard
cargo run --bin stack-server &
cd web && npm install && npm run dev
```

## License

Apache-2.0

## Acknowledgments

- [Graphite](https://graphite.dev) for pioneering the stacked diffs workflow
- [git-branchless](https://github.com/arxanas/git-branchless) for inspiration on Git internals
