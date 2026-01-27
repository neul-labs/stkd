# gt submit

Push branches to remote and create or update pull requests.

## Synopsis

```
gt submit [OPTIONS]
```

## Description

Pushes the current branch (or stack) to the remote repository and creates or updates pull requests. PRs are created with the correct base branch based on stack relationships.

## Options

| Option | Description |
|--------|-------------|
| `-s, --stack` | Submit entire stack (current + descendants) |
| `--draft` | Create PRs as drafts |
| `--push-only` | Only push, don't create/update PRs |
| `--no-push` | Only update PRs, don't push |
| `--update` | Update existing PR titles and descriptions |
| `-t, --title <TITLE>` | Custom PR title (single branch only) |
| `-b, --body <BODY>` | Custom PR body (single branch only) |
| `-r, --reviewers <USERS>` | Request reviewers (comma-separated usernames) |
| `-l, --labels <LABELS>` | Add labels (comma-separated) |
| `--template` | Use PR template from repository |
| `--only <BRANCHES>` | Only submit specific branches (comma-separated) |
| `--from <BRANCH>` | Submit from this branch to tip |
| `--to <BRANCH>` | Submit from root to this branch |
| `--dry-run` | Show what would be done without executing |

## Examples

### Submit Current Branch

```bash
# Push and create/update PR for current branch
gt submit

# Output:
# Pushing feature/auth...
# ✓ Pushed feature/auth
# Creating PR for feature/auth...
# ✓ Created PR #42 for feature/auth
#   https://github.com/owner/repo/pull/42
```

### Submit Entire Stack

```bash
# Push and create PRs for all branches in stack
gt submit --stack

# Output:
# Pushing 3 branch(es)...
# ✓ Pushed feature/models
# ✓ Pushed feature/api
# ✓ Pushed feature/ui
# Creating PRs...
# ✓ Created PR #42 for feature/models
# ✓ Created PR #43 for feature/api
# ✓ Created PR #44 for feature/ui
```

### Request Reviewers and Add Labels

```bash
# Submit with reviewers and labels
gt submit --reviewers alice,bob,carol --labels feature,urgent

# Output:
# ✓ Created PR #42 for feature/auth
#      Reviewers: alice, bob, carol
#      Labels: feature, urgent
```

### Use PR Template

```bash
# Use the repository's PR template
gt submit --template

# Stack looks for templates in:
# - .github/PULL_REQUEST_TEMPLATE.md
# - .github/pull_request_template.md
# - .github/PULL_REQUEST_TEMPLATE/default.md
# - .gitlab/merge_request_templates/Default.md
```

### Preview Changes (Dry Run)

```bash
# See what would happen without actually doing it
gt submit --dry-run

# Output:
# Dry run - showing what would be done:
#
#   → Push feature/auth to origin
#   → Create MR for feature/auth -> main
#        Reviewers: alice, bob
#        Labels: feature
#
# Run without --dry-run to execute
```

### Partial Submission

```bash
# Submit only specific branches
gt submit --only feature/step-1,feature/step-2

# Submit from a specific branch to the tip
gt submit --from feature/step-2

# Submit from root to a specific branch
gt submit --to feature/step-3
```

### Create Draft PRs

```bash
# Create as drafts for early feedback
gt submit --stack --draft
```

### Custom PR Title

```bash
# Set specific title
gt submit --title "Add user authentication"
```

### Update Existing PRs

```bash
# Update PR descriptions with latest stack info
gt submit --stack --update
```

## PR Description

Stack automatically adds a stack visualization to PR descriptions:

```markdown
---

## Stack

- #42
- **#43** (this MR)
- #44

---
*Managed by [Stack](https://github.com/neul-labs/stack)*
```

The current PR is highlighted in bold.

## Base Branch Selection

Stack sets PR base branches based on relationships:

```text
Stack:
  main → feature/models → feature/api → feature/ui

PRs created:
  feature/models → main
  feature/api → feature/models
  feature/ui → feature/api
```

## Push Behavior

- Uses `git push --force-with-lease` for safety
- Creates upstream tracking branch automatically (`-u`)
- Force-push is needed after restacking

## Related Commands

- [gt sync](./sync.md) - Sync and restack
- [gt land](./land.md) - Merge PRs
