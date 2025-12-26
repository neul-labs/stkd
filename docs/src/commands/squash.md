# gt squash

Squash commits in the current branch into a single commit.

## Usage

```bash
gt squash [OPTIONS]
```

## Description

The `squash` command combines multiple commits in the current branch into a single commit. This is useful for cleaning up your commit history before submitting a PR, or when you have multiple small commits that logically belong together.

The command works by:
1. Finding the merge base with the parent branch
2. Counting commits since that point
3. Soft resetting to before those commits
4. Creating a new commit with all changes

## Options

| Option | Description |
|--------|-------------|
| `-a, --all` | Squash all commits in the branch (from merge base) |
| `-n, --count <N>` | Number of commits to squash from HEAD |
| `-m, --message <MSG>` | New commit message (opens editor if not provided) |

## Examples

### Squash All Commits

```bash
# Squash all commits in the current branch
gt squash --all
```

This will combine all commits since the branch diverged from its parent into a single commit.

### Squash Last N Commits

```bash
# Squash last 3 commits
gt squash -n 3

# Squash last 2 commits with a message
gt squash -n 2 -m "Combined feature implementation"
```

### Interactive Squash

```bash
# Squash all commits, opening editor for message
gt squash --all
```

When no message is provided, your default git editor opens to compose the commit message.

## Workflow Example

```bash
# You have multiple commits in your branch
gt log
# ○ main
#   ◉ feature/add-auth
#       - Fix typo
#       - Add tests
#       - WIP auth
#       - Initial auth impl

# Squash them all before submitting
gt squash --all -m "Add authentication feature"

# Now you have a clean single commit
gt log
# ○ main
#   ◉ feature/add-auth
#       - Add authentication feature
```

## Notes

- The command will not squash if there's only one commit
- If you don't provide a message, your editor will open
- This is a destructive operation - the original commits are replaced
- Works best when you haven't pushed the branch yet

## See Also

- [gt fold](./fold.md) - Fold staged changes into a commit
- [gt split](./split.md) - Split a commit into multiple commits
- [gt modify](./modify.md) - Amend the current commit
