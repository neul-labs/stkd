# Stack

**Stacked Diffs for Git** - A Graphite-compatible CLI for managing stacked pull requests.

---

## What is Stack?

Stack helps you break large changes into small, reviewable pull requests that build on each other. Instead of one massive PR that's hard to review, you create a "stack" of focused changes.

```
main
 └── feature/auth-models      PR #1: Add user models
      └── feature/auth-api    PR #2: Add authentication API
           └── feature/auth-ui PR #3: Add login UI
```

Each branch depends on its parent, and Stack handles all the complexity of keeping them in sync.

## Why Stacked Diffs?

| Traditional PRs | Stacked Diffs |
|----------------|---------------|
| One large PR with many changes | Multiple small, focused PRs |
| Reviewers overwhelmed | Easy to review incrementally |
| All-or-nothing merging | Land changes as they're approved |
| Blocked waiting for review | Unblock yourself, keep coding |

## Quick Example

```bash
# Initialize Stack in your repo
gt init

# Create your first branch
gt create feature/add-models
# ... make changes, commit ...

# Stack another branch on top
gt create feature/add-api
# ... make more changes ...

# Submit all PRs at once
gt submit

# When the first PR is approved and merged
gt sync  # Automatically rebases the stack
```

## Features

- **GitHub & GitLab Support** - Full integration with both platforms
- **Smart Rebasing** - Automatically keeps your stack up to date
- **Undo/Redo** - Made a mistake? Just `gt undo`
- **Templates** - Create common stack patterns quickly
- **Watch Mode** - Auto-sync when PRs are merged

## Installation

=== "Cargo (Recommended)"

    ```bash
    cargo install stkd-cli
    ```

=== "From Source"

    ```bash
    git clone https://github.com/neul-labs/stkd
    cd stkd
    cargo install --path crates/stkd-cli
    ```

## Getting Started

1. **[Installation](getting-started/installation.md)** - Install Stack on your system
2. **[Quick Start](getting-started/quickstart.md)** - Get up and running in 5 minutes
3. **[First Stack](getting-started/first-stack.md)** - Create your first stack of PRs
4. **[Authentication](getting-started/authentication.md)** - Connect to GitHub or GitLab

## Need Help?

- Check the [FAQ](reference/faq.md) for common questions
- See [Troubleshooting](reference/troubleshooting.md) for solutions
- [Open an issue](https://github.com/neul-labs/stkd/issues) on GitHub
