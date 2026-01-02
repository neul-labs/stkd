# gt up / gt down

Navigate up or down the stack.

## Usage

```bash
gt up [count]
gt down [count]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[count]` | Number of branches to move (default: 1) |

## Examples

### Move One Branch

```bash
# Move toward the tip (child branch)
gt up

# Move toward the root (parent branch)
gt down
```

### Move Multiple Branches

```bash
# Move up 2 branches
gt up 2

# Move down 3 branches
gt down 3
```

## Stack Direction

```
main (trunk)
 └── feature/a     ← "down" direction (toward root)
      └── feature/b     ← current
           └── feature/c     ← "up" direction (toward tip)
```

- **Up** moves toward the **tip** (child branches, newer changes)
- **Down** moves toward the **root** (parent branches, trunk)

## Behavior

- Checks out the target branch
- If count exceeds available branches, moves to the end (top or bottom)

## Related Commands

- [`gt top`](top-bottom.md) - Jump to stack tip
- [`gt bottom`](top-bottom.md) - Jump to stack root
- [`gt checkout`](checkout.md) - Checkout by name
- [`gt log`](log.md) - View the stack
