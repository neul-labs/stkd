# gt restack

Restack branches onto their updated parent branches.

## Synopsis

```bash
gt restack [OPTIONS]
```

## Description

Rebases all tracked branches that need updating onto their current parents. This is useful when a parent branch has been modified and its children need to be rebased.

If a conflict is encountered, the operation pauses and you must resolve conflicts before running [`gt continue`](./continue.md).

## Options

| Option | Description |
|--------|-------------|
| `--current-only` | Only restack current branch and descendants |
| `--force` | Force restack even if branches appear up-to-date |

## Examples

```bash
# Restack all branches that need it
gt restack

# Only restack from current branch down
gt restack --current-only
```

## See Also

- [`gt sync`](./sync.md) — Sync with remote and restack
- [`gt continue`](./continue.md) — Continue after resolving conflicts
- [`gt abort`](./abort.md) — Abort the current operation
