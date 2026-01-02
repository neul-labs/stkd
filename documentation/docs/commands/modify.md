# gt modify

Amend the current commit with staged changes.

## Usage

```bash
gt modify [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `-m, --message <msg>` | New commit message |
| `--no-edit` | Keep existing commit message |

## Examples

### Amend with Staged Changes

```bash
# Stage your changes
git add .

# Amend the commit
gt modify
```

### Update Commit Message

```bash
gt modify -m "Better commit message"
```

### Keep Message, Add Changes

```bash
git add forgotten-file.txt
gt modify --no-edit
```

## Behavior

1. Amends the current HEAD commit with staged changes
2. Preserves authorship information
3. Updates the commit timestamp

!!! warning "Rebasing Downstream"
    After modifying a commit, you should run `gt restack` to update any child branches that depend on it.

## Workflow

Typical workflow when making changes:

```bash
# Make changes
vim src/feature.rs

# Stage changes
git add src/feature.rs

# Amend the commit
gt modify

# Update child branches
gt restack
```

## Related Commands

- [`gt restack`](restack.md) - Update child branches
- [`gt squash`](squash.md) - Squash multiple commits
- [`gt fold`](fold.md) - Fold into previous commit
