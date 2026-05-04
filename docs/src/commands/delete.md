# gt delete

Delete a tracked branch.

## Synopsis

```bash
gt delete [OPTIONS] <BRANCH>
```

## Description

Deletes a Git branch and removes it from Stack tracking. By default, refuses to delete branches that have children in the stack. Use `--force` to override.

## Arguments

| Argument | Description |
|----------|-------------|
| `<BRANCH>` | Name of the branch to delete |

## Options

| Option | Description |
|--------|-------------|
| `--force` | Delete even if the branch has children |

## Examples

```bash
# Delete a branch
gt delete feature/old-feature

# Force delete a branch with children
gt delete --force feature/old-feature
```

## See Also

- [`gt untrack`](./untrack.md) — Stop tracking without deleting
- [`gt rename`](./rename.md) — Rename the current branch
