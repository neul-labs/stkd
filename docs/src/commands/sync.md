# gt sync

Synchronize with remote and update local branches.

## Synopsis

```
gt sync [OPTIONS]
```

## Description

Fetches changes from the remote, updates the trunk branch, detects merged PRs, cleans up merged branches, and restacks dependent branches.

## Options

| Option | Description |
|--------|-------------|
| `--no-delete` | Don't delete merged branches |
| `--no-restack` | Don't rebase dependent branches |
| `--no-pull` | Don't update trunk |
| `--force` | Force restack even if not needed |

## Examples

### Basic Sync

```bash
# Full sync with remote
gt sync

# Output:
# Fetching from remote...
# ✓ Fetched from remote
# Updating main...
# ✓ Updated main
# Checking PR status...
#   → PR #42 was merged
# Cleaning up 1 merged branch(es)...
#   → Deleting feature/auth...
# ✓ Deleted feature/auth
# Restacking branches...
# ✓ Restacked feature/settings
# ✓ Sync complete
```

### Sync Without Cleanup

```bash
# Keep merged branches around
gt sync --no-delete
```

### Sync Without Restack

```bash
# Just fetch and check status
gt sync --no-restack
```

## What Sync Does

### 1. Fetch

Fetches latest changes from the remote:

```bash
git fetch origin --prune
```

### 2. Update Trunk

Updates your local trunk branch:

```bash
git checkout main
git pull --ff-only origin main
```

### 3. Check PR Status

Queries the provider (GitHub/GitLab) for each tracked branch:

- Detects merged PRs
- Detects closed PRs
- Updates local status

### 4. Clean Up Merged

For each merged PR:

1. Deletes the local branch
2. Removes tracking metadata
3. Updates parent-child relationships

### 5. Restack

Rebases branches that need updating:

```text
Before sync:
  main (old) → feature/base → feature/child

main updated, feature/base merged:
  main (new)

After restack:
  main (new) → feature/child (rebased)
```

## Conflict Resolution

If a conflict occurs during restacking:

```bash
gt sync

# Output:
# Restacking feature/child...
# CONFLICT (content): Merge conflict in src/main.rs
# Resolve conflicts and run 'gt continue'
```

To resolve:

```bash
# 1. Edit the conflicting file
vim src/main.rs

# 2. Mark as resolved
git add src/main.rs

# 3. Continue
gt continue

# Or abort
gt abort
```

## Best Practices

- Run `gt sync` at least once daily
- Run before starting new work
- Run before submitting for review
- Run after receiving review feedback

## Related Commands

- [gt submit](./submit.md) - Push and create PRs
- [gt land](./land.md) - Merge PRs
