# gt up / gt down

Navigate between branches in your stack.

## Synopsis

```
gt up
gt down
gt top
gt bottom
```

## Description

Move between branches in your current stack without typing branch names.

## Commands

### gt up

Move to the branch above (toward the tip of the stack):

```bash
# Stack:
# ○ feature/base
# ◉ feature/current  ← you are here
# ○ feature/next

gt up

# Now:
# ○ feature/base
# ○ feature/current
# ◉ feature/next  ← you are here
```

### gt down

Move to the branch below (toward the root of the stack):

```bash
# Stack:
# ○ feature/base
# ◉ feature/current  ← you are here
# ○ feature/next

gt down

# Now:
# ◉ feature/base  ← you are here
# ○ feature/current
# ○ feature/next
```

### gt top

Jump to the tip (topmost branch) of the stack:

```bash
gt top
# Switches to the last branch in the stack
```

### gt bottom

Jump to the root (first branch) of the stack:

```bash
gt bottom
# Switches to the first branch in the stack (after trunk)
```

## Examples

### Navigate Through Stack

```bash
# Go to the top of the stack
gt top

# Move down one branch at a time
gt down
gt down
gt down

# Jump back to the top
gt top
```

### Quick Review Flow

```bash
# Start at the bottom for review
gt bottom

# Review each branch's changes
git diff HEAD~1
gt up

git diff HEAD~1
gt up

# etc.
```

## Handling Edge Cases

### At the Top

```bash
gt up

# Output:
# Already at the top of the stack
```

### At the Bottom

```bash
gt down

# Output:
# Already at the bottom of the stack
# (Below this is 'main')
```

### Not on a Tracked Branch

```bash
gt up

# Output:
# Not on a tracked branch
# → Run 'gt track' to start tracking this branch
```

## Related Commands

- [gt log](./log.md) - View the stack
- [gt status](./status.md) - Current branch details

---

# gt info

Show information about a specific branch.

## Synopsis

```
gt info [BRANCH]
```

## Description

Displays detailed information about a branch. Defaults to the current branch if not specified.

## Arguments

| Argument | Description |
|----------|-------------|
| `BRANCH` | Branch name (optional, defaults to current) |

## Examples

```bash
# Info about current branch
gt info

# Info about specific branch
gt info feature/auth
```

## Output

```
Branch: feature/auth
Parent: main
Status: submitted
Children: feature/auth-ui, feature/auth-api
PR: #42
URL: https://github.com/owner/repo/pull/42
Created: 2024-01-15 10:30
Updated: 2024-01-15 14:22
```

---

# gt track

Track an existing branch with Stack.

## Synopsis

```
gt track [OPTIONS] [BRANCH]
```

## Description

Start tracking an existing Git branch with Stack. This is useful for branches created outside of Stack.

## Arguments

| Argument | Description |
|----------|-------------|
| `BRANCH` | Branch to track (defaults to current) |

## Options

| Option | Description |
|--------|-------------|
| `--parent <BRANCH>` | Specify the parent branch |

## Examples

```bash
# Track current branch
gt track

# Track with explicit parent
gt track --parent main feature/existing
```
