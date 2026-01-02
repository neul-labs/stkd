# gt delete

Delete a branch from the stack.

## Usage

```bash
gt delete [OPTIONS] [branch]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[branch]` | Branch to delete (default: current branch) |

## Options

| Option | Description |
|--------|-------------|
| `-f, --force` | Force deletion even if branch has unmerged changes |
| `-D` | Shorthand for `--force` |

## Examples

### Delete Current Branch

```bash
gt delete
```

### Delete Specific Branch

```bash
gt delete feature/old-feature
```

### Force Delete

Delete a branch with unmerged changes:

```bash
gt delete --force feature/abandoned
```

## Behavior

1. Checks if the branch has unmerged changes (unless `--force`)
2. Re-parents any child branches to the deleted branch's parent
3. Removes the branch from Stack's tracking
4. Deletes the Git branch
5. Switches to the parent branch if deleting current

## Re-parenting

When you delete a branch that has children, the children are re-parented:

Before:
```
main
 └── feature/a (delete this)
      └── feature/b
           └── feature/c
```

After:
```
main
 └── feature/b
      └── feature/c
```

## Related Commands

- [`gt create`](create.md) - Create a branch
- [`gt untrack`](track.md) - Stop tracking without deleting
