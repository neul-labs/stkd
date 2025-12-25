# Stack - Stacked Diffs for Git

A Graphite-compatible CLI for managing stacked pull requests on GitHub.

## Overview

Stack is an open-source alternative to [Graphite](https://graphite.dev) that brings the stacked diffs workflow to any Git repository. It helps you:

- **Break large changes into reviewable PRs** - Each logical change gets its own branch and PR
- **Keep dependent changes in sync** - When you update a base branch, Stack automatically rebases all dependent branches
- **Submit entire stacks at once** - Create/update PRs for your entire stack with one command

## Installation

```bash
# Build from source
cargo install --path crates/stack-cli

# The binary is named 'gt' for Graphite compatibility
gt --help
```

## Quick Start

```bash
# Initialize Stack in your repository
cd your-repo
gt init

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

# Submit all PRs
gt submit --stack

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
- `gt checkout [branch]` - Switch branches

### Stack Operations
- `gt log` - Show the current stack
- `gt info` - Show current branch info
- `gt status` - Show stack status

### Editing
- `gt modify [-m msg]` - Amend the current branch

### Synchronization
- `gt sync` - Sync with remote and restack
- `gt restack` - Rebase stack onto updated parents
- `gt submit [--stack]` - Create/update PRs
- `gt land` - Merge the stack

### Conflict Resolution
- `gt continue` - Continue after resolving conflicts
- `gt abort` - Abort the current operation

### Configuration
- `gt auth --token <token>` - Authenticate with GitHub
- `gt config [key] [value]` - View/edit configuration

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

## Comparison with Graphite

| Feature | Graphite | Stack |
|---------|----------|-------|
| Open source | ❌ | ✅ |
| Self-hosted | ❌ | ✅ |
| CLI commands | `gt` | `gt` (compatible) |
| GitHub support | ✅ | ✅ |
| GitLab support | ❌ | 🔜 Planned |
| Web dashboard | ✅ | ❌ |
| AI PR descriptions | ✅ (paid) | 🔜 Optional |
| VCS integration | ❌ | ✅ |

## Integration with VCS

Stack can optionally integrate with [VCS](https://github.com/dipankar/vcs) for teams working with large assets and ML workflows:

```toml
# .git/stack/config.json
{
  "vcs": {
    "enabled": true,
    "share_intent": true,
    "respect_policy_gates": true
  }
}
```

When enabled:
- Commits include intent metadata (who made the change, why)
- Stack respects VCS policy gates before landing
- Large asset changes are tracked properly

## Development

```bash
# Clone the repository
git clone https://github.com/dipankar/stack
cd stack

# Build
cargo build

# Run tests
cargo test

# Run the CLI
cargo run --bin gt -- --help
```

## License

Apache-2.0

## Acknowledgments

- [Graphite](https://graphite.dev) for pioneering the stacked diffs workflow
- [git-branchless](https://github.com/arxanas/git-branchless) for inspiration on Git internals
