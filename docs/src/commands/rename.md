# gt rename

Rename the current branch.

## Synopsis

```bash
gt rename <NAME>
```

## Description

Renames the current Git branch and updates Stack metadata, including parent-child relationships. The branch must be tracked by Stack.

## Arguments

| Argument | Description |
|----------|-------------|
| `<NAME>` | New name for the branch |

## Examples

```bash
# Rename current branch
gt rename feature/new-auth
```

## See Also

- [`gt create`](./create.md) — Create a new stacked branch
- [`gt delete`](./delete.md) — Delete a branch
