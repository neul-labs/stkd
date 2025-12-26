# gt track

Start tracking an existing branch with Stack.

## Synopsis

```
gt track [OPTIONS] [BRANCH]
```

## Description

Track an existing Git branch with Stack. This adds the branch to Stack's metadata so it can participate in stacking operations.

## Arguments

| Argument | Description |
|----------|-------------|
| `BRANCH` | Branch to track (defaults to current branch) |

## Options

| Option | Description |
|--------|-------------|
| `--parent <BRANCH>` | Specify the parent branch |

## Examples

### Track Current Branch

```bash
git checkout feature/existing-branch
gt track

# Output:
# Tracking 'feature/existing-branch'
# Parent: main (auto-detected)
```

### Track with Explicit Parent

```bash
gt track --parent feature/base feature/child

# Output:
# Tracking 'feature/child'
# Parent: feature/base
```

### Track a Series of Branches

```bash
# Track an existing chain
gt track --parent main feature/step-1
gt track --parent feature/step-1 feature/step-2
gt track --parent feature/step-2 feature/step-3
```

## Parent Detection

When no parent is specified, Stack attempts to auto-detect:

1. Looks for a merge base with trunk
2. Checks if the branch is based on another tracked branch
3. Falls back to trunk if uncertain

```bash
gt track

# Output:
# Tracking 'feature/branch'
# Parent: main (auto-detected from merge base)
```

If detection fails:

```bash
gt track

# Output:
# Could not determine parent for 'feature/branch'
# Use --parent to specify the parent branch
```

## When to Use Track

Use `gt track` when:

- You created a branch with `git checkout -b` instead of `gt create`
- You're adopting Stack in an existing workflow
- You're incorporating a colleague's branch into your stack
- You want to manage a branch created from a PR

## Difference from Create

| Command | Creates Branch | Tracks |
|---------|---------------|--------|
| `gt create` | Yes | Yes |
| `gt track` | No | Yes |

```bash
# These are equivalent:
git checkout -b feature/new && gt track

gt create feature/new
```

## Related Commands

- [gt create](./create.md) - Create and track a new branch
- [gt untrack](./navigation.md) - Stop tracking a branch
