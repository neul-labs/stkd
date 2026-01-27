# Frequently Asked Questions

## General

### What is Stack?

Stack is a command-line tool for managing stacked pull requests (stacked diffs) in Git repositories. It helps you break large features into small, reviewable pieces that can be developed, reviewed, and merged incrementally.

### Why "gt" instead of "stack"?

The command is `gt` (short for "git-tree" or "graphite-like tool"). It's short to type and doesn't conflict with other common commands.

### How is Stack different from git-town, graphite, etc.?

Stack is designed to be:
- **Simple**: Focused on core stacking operations
- **Offline-first**: Works fully offline, syncs when you want
- **Multi-provider**: Supports GitHub, GitLab, and more
- **Open source**: Free to use and modify

## Usage

### How do I start using Stack in an existing project?

```bash
# Initialize (auto-detects settings)
gt init

# Authenticate
gt auth login --github

# Start stacking
gt create my-feature
```

### Can I use Stack with existing branches?

Yes! Use `gt track` to start tracking existing branches:

```bash
git checkout my-existing-branch
gt track --parent main
```

### What happens to my commits during restacking?

Commits are rebased (reapplied) onto the updated parent. The changes are preserved, but commit hashes will change.

### Can I use Stack without creating PRs?

Yes! Stack works for local branch management too. Just don't run `gt submit` until you're ready for PRs.

## Troubleshooting

### "Not authenticated" error

Run authentication:

```bash
gt auth login --github
# or
gt auth login --gitlab
```

### "Branch not tracked" error

Track the branch first:

```bash
gt track
```

### Conflicts during sync

Resolve conflicts manually:

```bash
# Edit conflicting files
vim src/file.rs

# Mark resolved
git add src/file.rs

# Continue
gt continue
```

### PRs created with wrong base

Update the PR base:

```bash
gt submit --update
```

### Force push needed

After restacking, you'll need to force push:

```bash
gt submit  # Uses --force-with-lease internally
```

## Provider-Specific

### GitHub

#### "Resource not accessible" error

Your token may be missing scopes. Generate a new token with:
- `repo` scope
- `read:org` scope (for org repos)

#### Rate limiting

GitHub has API rate limits. If you hit them:
- Wait for reset (usually 1 hour)
- Use a token with higher limits (GitHub Pro/Enterprise)

### GitLab

#### Can't find project

Ensure the project path is correct:

```toml
[provider]
owner = "group/subgroup"  # Full path
repo = "project"
```

#### Pipeline status not showing

Ensure GitLab CI is configured (`.gitlab-ci.yml` exists).

## Advanced

### Can I have multiple stacks?

Yes! Create branches from main for each independent stack:

```bash
# Stack 1
git checkout main
gt create feature-a/base

# Stack 2
git checkout main
gt create feature-b/base
```

### Can I move a branch to a different parent?

Yes:

```bash
gt rebase --onto new-parent
```

### Can I split a branch into multiple?

Git's interactive rebase can help:

```bash
# Interactive rebase to split commits
git rebase -i parent-branch

# Then create new branches at appropriate commits
```

### How do I abandon a stack?

Close PRs and delete branches:

```bash
# Close PRs on GitHub/GitLab
# Then locally:
git checkout main
git branch -D feature-a feature-b feature-c
gt sync  # Cleans up tracking
```

## Contributing

### How can I contribute?

See [Contributing](./contributing/README.md) for guidelines.

### Where do I report bugs?

Open an issue on [GitHub](https://github.com/neul-labs/stack/issues).

### How do I request a feature?

Open an issue with the "feature request" label.
