# gt top / gt bottom

Jump to the tip or root of the current stack.

## Usage

```bash
gt top
gt bottom
```

## Examples

```bash
# Jump to the newest branch (tip)
gt top

# Jump to the oldest branch (root)
gt bottom
```

## Stack Position

```
main (trunk)
 └── feature/a     ← bottom (root)
      └── feature/b
           └── feature/c     ← top (tip)
```

## Behavior

- **top**: Checks out the branch at the tip of the stack (furthest from trunk)
- **bottom**: Checks out the branch at the root of the stack (closest to trunk)

Both commands only move within the current stack. They won't jump to trunk or unrelated branches.

## Related Commands

- [`gt up`](navigation.md) - Move up one branch
- [`gt down`](navigation.md) - Move down one branch
- [`gt checkout`](checkout.md) - Checkout by name
