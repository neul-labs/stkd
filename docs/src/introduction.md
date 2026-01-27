# Stack - Stacked Diffs for Git

**Stack** is a command-line tool that makes it easy to work with stacked pull requests (also known as stacked diffs) in Git repositories.

## What is Stack?

Stack helps you break down large features into smaller, reviewable pull requests while maintaining the dependencies between them. Instead of creating one massive PR that's hard to review, you create a *stack* of small, focused PRs that build on each other.

```text
main
 └── feature/add-user-model      (PR #1)
      └── feature/add-user-api   (PR #2)
           └── feature/add-user-ui (PR #3)
```

## Why Use Stacked Diffs?

### For Authors

- **Faster Reviews**: Smaller PRs are reviewed faster
- **Logical Organization**: Group related changes together
- **Parallel Development**: Start the next change while waiting for review
- **Easier Debugging**: Isolated changes are easier to test and revert

### For Reviewers

- **Focused Reviews**: Each PR has a single purpose
- **Better Context**: Changes are organized in logical order
- **Reduced Cognitive Load**: Smaller diffs are easier to understand
- **Incremental Understanding**: Build knowledge step by step

## Key Features

- **Stack Management**: Create, navigate, and manage branch dependencies
- **Automatic Restacking**: Keep your stack up-to-date when changes are merged
- **PR Integration**: Create and update PRs with stack visualization
- **Multi-Provider**: Support for GitHub, GitLab, and more
- **Conflict Resolution**: Tools to help resolve rebase conflicts

## Quick Example

```bash
# Start a new feature stack
gt create feature/auth-base

# Make some changes and commit
git add . && git commit -m "Add authentication types"

# Stack another change on top
gt create feature/auth-handler

# More changes
git add . && git commit -m "Add auth middleware"

# Push and create PRs for the entire stack
gt submit --stack

# View your stack
gt log
# ┌ ○ feature/auth-base [submitted] (#42)
# └ ◉ feature/auth-handler [submitted] (#43)
```

## Getting Started

Ready to try Stack? Head to the [Installation](./installation/README.md) guide to get started, or jump straight to [Your First Stack](./getting-started/first-stack.md) if you've already installed it.

## Supported Platforms

Stack supports the following Git hosting providers:

| Provider | Status | Features |
|----------|--------|----------|
| GitHub | Stable | Full support |
| GitLab | Beta | Core features |
| Gitea | Planned | - |

## Getting Help

- **Documentation**: You're reading it!
- **Issues**: [GitHub Issues](https://github.com/neul-labs/stkd/issues)
- **Discussions**: [GitHub Discussions](https://github.com/neul-labs/stkd/discussions)
