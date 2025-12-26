# gt log

Display the current stack structure.

## Synopsis

```
gt log [OPTIONS]
```

## Description

Shows a visual representation of your current stack, including branch relationships, PR status, and the current position.

## Options

| Option | Description |
|--------|-------------|
| `-a, --all` | Show all tracked branches, not just current stack |

## Examples

### View Current Stack

```bash
gt log

# Output:
# ┌ ○ feature/models [submitted] (#42)
# │ ○ feature/api [submitted] (#43)
# └ ◉ feature/ui [active] (#44)
```

### View All Branches

```bash
gt log --all

# Output:
# Trunk: main
#
# ├─ ○ feature/models (#42)
# │  └─ ○ feature/api (#43)
# │     └─ ◉ feature/ui (#44) [active]
# │
# └─ ○ feature/settings (#45)
#    ├─ ○ feature/settings-ui (#46)
#    └─ ○ feature/settings-api (#47)
```

## Output Format

### Markers

| Symbol | Meaning |
|--------|---------|
| `◉` | Current branch |
| `○` | Other branch in stack |

### Status Indicators

| Status | Meaning |
|--------|---------|
| `[active]` | Branch is being worked on |
| `[submitted]` | PR has been created |
| `[merged]` | PR has been merged |
| `[closed]` | PR was closed |

### PR Numbers

PR numbers are shown in parentheses:

```
feature/auth (#42)
```

### PR URLs

In verbose mode, URLs are shown below the branch:

```
○ feature/auth (#42)
│   https://github.com/owner/repo/pull/42
```

## Stack Visualization

The tree structure shows branch relationships:

```
┌ ○ feature/base     ← root (first branch from trunk)
│ ○ feature/middle   ← middle of stack
└ ◉ feature/tip      ← tip (last branch, current)
```

For branching stacks:

```
├─ ○ feature/base
│  ├─ ○ feature/a
│  │  └─ ○ feature/a2
│  └─ ○ feature/b
```

## Integration with Git

You can combine with Git commands:

```bash
# Show stack with git log
gt log && echo && git log --oneline -5

# Show stack with git status
gt log && echo && git status -s
```

## Related Commands

- [gt status](./status.md) - Detailed branch status
- [gt info](./navigation.md) - Info about specific branch
