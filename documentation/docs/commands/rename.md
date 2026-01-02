# gt rename

Rename the current branch.

## Usage

```bash
gt rename <new-name>
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<new-name>` | New name for the branch |

## Examples

### Basic Rename

```bash
gt rename feature/better-name
```

## Behavior

1. Renames the Git branch locally
2. Updates Stack's tracking metadata
3. If a PR exists, updates the PR's source branch (provider-dependent)

!!! note "Remote Branches"
    Renaming does not automatically rename the remote branch. After renaming, you may need to push the new branch and delete the old one.

## Related Commands

- [`gt create`](create.md) - Create a branch
- [`gt delete`](delete.md) - Delete a branch
