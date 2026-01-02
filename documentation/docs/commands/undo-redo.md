# gt undo / gt redo

Undo or redo Stack operations.

## Usage

```bash
gt undo [count]
gt redo [count]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[count]` | Number of operations to undo/redo (default: 1) |

## Examples

### Undo Last Operation

```bash
gt undo
```

### Undo Multiple Operations

```bash
gt undo 3
```

### Redo Last Undo

```bash
gt redo
```

## What Can Be Undone

Stack tracks these operations:

- `gt create` - Undo creates delete
- `gt delete` - Undo restores the branch
- `gt rename` - Undo restores old name
- `gt modify` - Undo restores previous commit
- `gt restack` - Undo restores previous state
- `gt squash` - Undo restores original commits

## History Limit

Stack keeps history of the last 50 operations per repository.

## Viewing History

Currently, there's no command to view history. Undo operations are based on chronological order.

## Limitations

Some operations cannot be undone:

- Operations after `git gc`
- Changes made outside of Stack commands
- Very old operations (beyond history limit)

!!! tip "Safe Experimentation"
    The undo feature makes it safe to experiment. Try a restack, and if it doesn't work out, just `gt undo`.

## Related Commands

- All Stack commands that modify state support undo
