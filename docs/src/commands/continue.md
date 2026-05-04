# gt continue

Continue after resolving conflicts.

## Synopsis

```bash
gt continue
```

## Description

Resumes an operation (such as `gt restack` or `gt sync`) that was paused due to a rebase conflict. Before running this command, you must resolve all conflicts and stage your changes.

If no operation is in progress, reports that fact and exits successfully.

## Examples

```bash
# Resolve conflicts, stage changes, then continue
git add .
gt continue
```

## See Also

- [`gt abort`](./abort.md) — Abort the current operation
- [`gt restack`](./restack.md) — Restack branches
- [`gt sync`](./sync.md) — Sync with remote
