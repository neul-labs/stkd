# gt squash

Squash commits in the current branch.

## Usage

```bash
gt squash [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `-m, --message <msg>` | Message for squashed commit |
| `-n, --count <n>` | Number of commits to squash |
| `--all` | Squash all commits since parent |

## Examples

### Squash All Commits

Squash all commits in the current branch into one:

```bash
gt squash --all
```

### Squash Last N Commits

```bash
gt squash -n 3
```

### With Custom Message

```bash
gt squash --all -m "Feature: Add authentication"
```

## Behavior

1. Identifies commits to squash
2. Creates a single commit with combined changes
3. Updates the branch HEAD

!!! warning "Rebasing Required"
    After squashing, run `gt restack` to update any child branches.

## When to Use

- Before submitting a PR to clean up WIP commits
- To combine related commits into logical units
- To simplify history before landing

## Related Commands

- [`gt modify`](modify.md) - Amend single commit
- [`gt fold`](fold.md) - Fold changes into previous commit
- [`gt restack`](restack.md) - Update child branches
