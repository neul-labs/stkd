# gt checkout

Checkout a branch in the stack.

## Synopsis

```bash
gt checkout <BRANCH>
```

## Description

Checks out a specific branch by name. This is a convenience wrapper around `git checkout` that works within the context of your stack.

## Arguments

| Argument | Description |
|----------|-------------|
| `<BRANCH>` | Name of the branch to checkout |

## Examples

```bash
# Checkout a specific branch
gt checkout feature/step-2
```

## See Also

- [`gt up`](./navigation.md) — Move up the stack
- [`gt down`](./navigation.md) — Move down the stack
