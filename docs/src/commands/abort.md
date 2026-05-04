# gt abort

Abort the current operation.

## Synopsis

```bash
gt abort
```

## Description

Aborts an in-progress operation (such as `gt restack` or `gt sync`). This resets the repository to its state before the operation began and cleans up any Stack metadata.

If no operation is in progress, reports that fact and exits successfully.

## Examples

```bash
# Abort the current operation
gt abort
```

## See Also

- [`gt continue`](./continue.md) — Continue after resolving conflicts
- [`gt restack`](./restack.md) — Restack branches
