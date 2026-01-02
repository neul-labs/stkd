# gt checkout

Switch to a specific branch.

## Usage

```bash
gt checkout <branch>
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<branch>` | Name of branch to checkout |

## Examples

### Checkout by Name

```bash
gt checkout feature/login
```

### Checkout Trunk

```bash
gt checkout main
```

## Behavior

Checks out the specified branch. Works with both tracked and untracked branches.

## Fuzzy Matching

Stack supports partial branch names:

```bash
gt checkout login
# Matches: feature/login, fix/login-bug, etc.
```

If multiple branches match, you'll be prompted to select one.

## Related Commands

- [`gt up`](navigation.md) - Move up the stack
- [`gt down`](navigation.md) - Move down the stack
- [`gt top`](top-bottom.md) - Jump to stack tip
- [`gt bottom`](top-bottom.md) - Jump to stack root
