# Getting Started

Welcome to Stack! This guide will help you get up and running with stacked diffs in just a few minutes.

## Prerequisites

Before you begin:

1. **Install Stack**: Follow the [Installation Guide](../installation/README.md)
2. **Have a Git repository**: Any Git repo will work
3. **Configure your provider**: Set up [GitHub](../providers/github.md) or [GitLab](../providers/gitlab.md)

## Authenticate

First, authenticate with your Git hosting provider:

```bash
# For GitHub
gt auth

# This opens a browser for OAuth authentication
# Follow the prompts to complete setup
```

## Your First Stack

Let's create a simple stack:

```bash
# Start from your main branch
git checkout main
git pull

# Create your first stacked branch
gt create feature/step-1

# Make some changes
echo "Step 1 implementation" > step1.txt
git add step1.txt
git commit -m "Add step 1"

# Stack another branch on top
gt create feature/step-2

# Make more changes
echo "Step 2 implementation" > step2.txt
git add step2.txt
git commit -m "Add step 2"

# View your stack
gt log
```

You'll see output like:

```text
┌ ○ feature/step-1 [active]
└ ◉ feature/step-2 [active]
```

## Submit Your Stack

Push your changes and create PRs:

```bash
# Submit the entire stack
gt submit --stack

# Or submit just the current branch
gt submit
```

Stack will:
1. Push each branch to the remote
2. Create PRs with the correct base branches
3. Add stack visualization to PR descriptions

## Navigate Your Stack

Move between branches:

```bash
# Go to the branch below (toward main)
gt down

# Go to the branch above (toward tip)
gt up

# Go to a specific branch
git checkout feature/step-1
```

## Keep Your Stack Updated

When PRs get merged:

```bash
# Sync with remote and update your stack
gt sync
```

This will:
1. Fetch the latest changes
2. Detect merged PRs
3. Restack remaining branches onto the updated main

## Next Steps

- [Your First Stack](./first-stack.md) - Detailed walkthrough
- [Common Workflows](./workflows.md) - Real-world patterns
- [Command Reference](../commands/README.md) - All available commands

## Quick Reference

| Command | Description |
|---------|-------------|
| `gt create <name>` | Create a new stacked branch |
| `gt submit` | Push and create/update PR |
| `gt submit --stack` | Submit entire stack |
| `gt sync` | Sync with remote |
| `gt log` | View current stack |
| `gt up` / `gt down` | Navigate stack |
| `gt land` | Merge current PR |
