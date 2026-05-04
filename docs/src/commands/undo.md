# gt undo

Undo the last Stack operation.

## Synopsis

```bash
gt undo [OPTIONS]
```

## Description

Reverses the most recently performed Stack operation. Operations are recorded in a history log, and `gt undo` replays the inverse action.

## Options

| Option | Description |
|--------|-------------|
| `--steps <N>` | Number of operations to undo (default: 1) |

## Examples

```bash
# Undo the last operation
gt undo

# Undo the last 3 operations
gt undo --steps 3
```

## See Also

- [`gt redo`](./redo.md) — Redo a previously undone operation
