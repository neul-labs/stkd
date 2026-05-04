# gt info

Show information about the current branch.

## Synopsis

```bash
gt info [OPTIONS]
```

## Description

Displays metadata for the current branch, including its parent, children, associated merge request, and status in the stack.

## Options

| Option | Description |
|--------|-------------|
| `--json` | Output as JSON |

## Examples

```bash
# Show branch info
gt info

# Output as JSON
gt info --json
```

## See Also

- [`gt status`](./status.md) — Show repository status
- [`gt log`](./log.md) — Visualize the stack
