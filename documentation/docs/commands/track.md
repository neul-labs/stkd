# gt track / gt untrack

Start or stop tracking branches in the stack.

## Track

### Usage

```bash
gt track [OPTIONS] [branch]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `[branch]` | Branch to track (default: current) |

### Options

| Option | Description |
|--------|-------------|
| `--parent <branch>` | Set the parent branch |

### Examples

Track an existing branch:

```bash
gt track feature/existing-branch --parent main
```

## Untrack

### Usage

```bash
gt untrack [branch]
```

### Arguments

| Argument | Description |
|----------|-------------|
| `[branch]` | Branch to untrack (default: current) |

### Examples

Stop tracking a branch (keeps Git branch):

```bash
gt untrack feature/old-branch
```

## When to Use

**Track**: When you have an existing branch you want to include in a stack.

**Untrack**: When you want to remove a branch from Stack's tracking without deleting the Git branch.

## Related Commands

- [`gt create`](create.md) - Create and track a new branch
- [`gt delete`](delete.md) - Delete and untrack a branch
