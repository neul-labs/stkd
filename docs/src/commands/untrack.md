# gt untrack

Stop tracking a branch without deleting it.

## Synopsis

```bash
gt untrack <BRANCH>
```

## Description

Removes a branch from Stack tracking while keeping the Git branch intact. Children's parent references are updated to point to the removed branch's parent.

## Arguments

| Argument | Description |
|----------|-------------|
| `<BRANCH>` | Name of the branch to untrack |

## Examples

```bash
# Untrack a branch
gt untrack feature/temp
```

## See Also

- [`gt track`](./track.md) — Start tracking an existing branch
- [`gt delete`](./delete.md) — Delete a tracked branch
