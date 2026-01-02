# Resolving Conflicts

How to handle merge conflicts when using Stack.

## When Conflicts Occur

Conflicts can happen during:

- `gt sync` - When remote has changes
- `gt restack` - When rebasing onto updated parents
- `gt land` - When merging PRs

## Basic Conflict Resolution

When Stack encounters a conflict:

```bash
$ gt sync
Syncing with remote...
CONFLICT: feature/my-branch has conflicts

# Resolve conflicts in your editor
$ git status
# Shows conflicted files

# Edit files to resolve conflicts
$ vim src/conflicted-file.rs

# Mark as resolved
$ git add src/conflicted-file.rs

# Continue the operation
$ gt continue
```

## The Conflict Workflow

```
┌─────────────┐
│  gt sync    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  CONFLICT   │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│ Fix conflicts   │
│ git add <files> │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
┌────────┐ ┌────────┐
│continue│ │ abort  │
└────────┘ └────────┘
```

## Aborting

If you can't resolve conflicts or want to start over:

```bash
gt abort
```

This restores the repository to its state before the operation.

## Common Conflict Scenarios

### Conflict with Trunk

Someone merged changes to main that conflict with your branch:

```bash
gt sync
# CONFLICT in feature/my-branch

# Option 1: Resolve and continue
vim conflicted-file
git add conflicted-file
gt continue

# Option 2: Abort and rethink
gt abort
```

### Conflict in Dependent Branch

You modified a parent branch, and child has conflicts:

```bash
# Modify parent
gt checkout feature/parent
gt modify
gt restack
# CONFLICT in feature/child

# Resolve in child context
vim conflicted-file
git add conflicted-file
gt continue
```

### Conflict During Land

The PR can't be merged cleanly:

```bash
gt land feature/my-branch
# ERROR: Merge conflicts detected

# Sync first to get latest changes
gt sync
# Resolve any conflicts
gt continue

# Try landing again
gt land feature/my-branch
```

## Prevention Tips

### Sync Often

```bash
# Start of day
gt checkout main
gt sync

# Before submitting
gt sync
gt submit
```

### Keep Branches Focused

Smaller, focused branches have fewer conflicts:

```
# Bad: One branch touching everything
feature/big-refactor  # Touches 50 files

# Good: Split into focused branches
feature/refactor-models  # 5 files
feature/refactor-api     # 10 files
feature/refactor-ui      # 8 files
```

### Communicate with Team

- Let teammates know when rebasing shared branches
- Coordinate who's working on which files
- Review and land PRs promptly

## Advanced: Manual Rebase

If Stack's rebase isn't working as expected:

```bash
# Abort Stack's operation
gt abort

# Manual rebase
git checkout feature/my-branch
git rebase origin/main

# Resolve conflicts
# ...

# Update Stack's tracking
gt sync
```

## Getting Help

If you're stuck:

```bash
# See current state
git status

# See what operation is in progress
gt status

# Abort and try again
gt abort
```
