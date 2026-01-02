# gt restack

Rebase branches onto their updated parents.

## Usage

```bash
gt restack [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Show what would happen without making changes |

## Examples

### Restack All Branches

```bash
gt restack
```

### Preview Changes

```bash
gt restack --dry-run
```

## When to Use

Run `gt restack` after:

- Modifying a commit (`gt modify`)
- Squashing commits (`gt squash`)
- Merging changes from trunk
- Any time parent commits have changed

## Behavior

1. Identifies branches that need rebasing
2. Rebases each branch onto its parent in topological order
3. Reports success or conflicts

## Handling Conflicts

If conflicts occur:

```bash
# Resolve conflicts
git add <resolved-files>

# Continue restacking
gt continue

# Or abort
gt abort
```

## Difference from Sync

| Command | Fetches Remote | Rebases |
|---------|----------------|---------|
| `gt sync` | Yes | Yes |
| `gt restack` | No | Yes |

Use `gt sync` to update from remote; use `gt restack` for local-only rebasing.

## Related Commands

- [`gt sync`](sync.md) - Fetch and restack
- [`gt modify`](modify.md) - Amend commits
- [`gt continue`](../commands/index.md) - Continue after conflicts
