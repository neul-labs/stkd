# gt fold

Fold staged changes into a previous commit.

## Usage

```bash
gt fold [OPTIONS]
```

## Description

The `fold` command takes your staged changes and folds them into an existing commit. This is useful when you want to add forgotten changes to a previous commit without creating a new commit.

By default, it folds changes into HEAD. You can also specify a different target commit using `--into`.

## Options

| Option | Description |
|--------|-------------|
| `-i, --into <COMMIT>` | Target commit to fold into (default: HEAD) |
| `--fixup` | Create a fixup commit instead of immediately folding |

## Examples

### Fold into HEAD

```bash
# Make some changes
echo "fix" >> file.rs

# Stage the changes
git add file.rs

# Fold into the current commit
gt fold
```

### Fold into a Specific Commit

```bash
# Stage your changes
git add file.rs

# Create a fixup commit targeting HEAD~2
gt fold --into HEAD~2 --fixup

# Later, run rebase to apply the fixup
git rebase -i --autosquash
```

### Create a Fixup Commit

```bash
# Stage changes
git add .

# Create a fixup commit (doesn't rebase immediately)
gt fold --fixup

# The fixup commit is created, ready for autosquash
```

## How It Works

1. **Without `--fixup`**: Creates a fixup commit and immediately runs an autosquash rebase
2. **With `--fixup`**: Creates a fixup commit but leaves it for you to rebase later

The autosquash rebase uses `GIT_SEQUENCE_EDITOR=true` to automatically accept the rebase todo, so the operation is non-interactive.

## Workflow Example

```bash
# Working on a feature, you notice a bug in an earlier commit
gt log
# ○ main
#   ○ feature/step-1  <- forgot to add a file here
#     ◉ feature/step-2

# Add the forgotten file
git add forgotten-file.rs

# Navigate to the target branch
gt down
gt fold

# The change is now part of feature/step-1
# Restack to update dependent branches
gt restack
```

## Notes

- You must have staged changes to fold
- If no changes are staged, the command will warn you
- When using `--into` with an older commit, prefer `--fixup` to avoid conflicts
- The autosquash rebase may encounter conflicts if changes overlap

## See Also

- [gt squash](./squash.md) - Squash multiple commits into one
- [gt split](./split.md) - Split a commit into multiple commits
- [gt modify](./modify.md) - Amend the current commit
