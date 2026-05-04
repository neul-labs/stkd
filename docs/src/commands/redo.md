# gt redo

Redo a previously undone operation.

## Synopsis

```bash
gt redo [OPTIONS]
```

## Description

Reapplies the most recently undone Stack operation. The redo history is cleared when a new operation is performed.

## Options

| Option | Description |
|--------|-------------|
| `--steps <N>` | Number of operations to redo (default: 1) |

## Examples

```bash
# Redo the last undone operation
gt redo
```

## See Also

- [`gt undo`](./undo.md) — Undo the last Stack operation
