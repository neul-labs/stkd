# gt create

Create a new stacked branch based on the current branch.

## Synopsis

```
gt create [OPTIONS] <NAME>
```

## Description

Creates a new Git branch and tracks it as a child of the current branch. The new branch will be checked out automatically.

## Arguments

| Argument | Description |
|----------|-------------|
| `NAME` | Name of the new branch to create |

## Options

| Option | Description |
|--------|-------------|
| `-m, --message <MSG>` | Initial commit message (creates empty commit) |
| `--no-checkout` | Create branch but don't switch to it |

## Examples

### Basic Usage

```bash
# Create a new branch based on current branch
gt create feature/auth

# Output:
# Created branch 'feature/auth' based on 'main'
# Switched to branch 'feature/auth'
```

### Create with Initial Commit

```bash
# Create with a placeholder commit
gt create feature/api --message "WIP: API implementation"
```

### Create Without Switching

```bash
# Stay on current branch
gt create feature/later --no-checkout
```

## Branch Naming

Stack works with any branch naming convention:

```bash
# Feature branches
gt create feature/user-auth
gt create feature/payment-flow

# Bug fixes
gt create fix/login-error
gt create bugfix/123

# Experiments
gt create experiment/new-algorithm

# Personal namespacing
gt create jdoe/feature-x
```

## How It Works

1. Creates a new Git branch at HEAD
2. Records the current branch as the parent
3. Switches to the new branch (unless `--no-checkout`)

```text
Before:
  main ──A──B──C (HEAD)

After `gt create feature/x`:
  main ──A──B──C
              │
  feature/x ──┘ (HEAD)
```

## Related Commands

- [gt track](./track.md) - Track an existing branch
- [gt log](./log.md) - View the stack
