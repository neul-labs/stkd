# gt split

Split the current commit into multiple commits.

## Usage

```bash
gt split [OPTIONS]
```

## Description

The `split` command helps you break a large commit into smaller, more focused commits. This is useful when:

- A commit contains logically separate changes
- You want to make your commit history more granular
- A reviewer asks you to split a large PR

The command uses interactive staging (`git add -p`) to let you choose which changes go into each commit.

## Options

| Option | Description |
|--------|-------------|
| `-c, --count <N>` | Number of commits to create (default: 2) |

## Examples

### Split into 2 Commits

```bash
# Split the current commit into 2
gt split

# Interactive session:
# 1. Select hunks for first commit
# 2. Write commit message for first commit
# 3. Remaining changes go into second commit
# 4. Write commit message for second commit
```

### Split into Multiple Commits

```bash
# Split into 3 commits
gt split -c 3
```

## How It Works

1. Soft resets HEAD~1 to preserve changes in the working directory
2. Unstages all files
3. For each commit (except the last):
   - Opens `git add -p` for interactive staging
   - Opens your editor for the commit message
4. For the last commit:
   - Stages all remaining changes
   - Opens your editor for the commit message

## Interactive Staging

During `git add -p`, you'll see each hunk and can:

| Key | Action |
|-----|--------|
| `y` | Stage this hunk |
| `n` | Don't stage this hunk |
| `s` | Split into smaller hunks |
| `e` | Edit this hunk |
| `q` | Quit (stage selected hunks) |
| `?` | Help |

## Workflow Example

```bash
# You have a commit with both refactoring and new feature
gt log
# ○ main
#   ◉ feature/mixed-changes
#       "Add auth and refactor utils"

# Split it into logical commits
gt split -c 2

# First commit: select refactoring hunks
# git add -p shows each change
# Stage the refactoring parts, skip feature parts

# Write commit message: "Refactor utils module"

# Second commit: remaining changes auto-staged
# Write commit message: "Add authentication feature"

# Result
gt log
# ○ main
#   ○ feature/mixed-changes
#       "Refactor utils module"
#   ◉ feature/mixed-changes~split
#       "Add authentication feature"
```

## Notes

- Cannot split into fewer than 2 commits
- The original commit is replaced by the new commits
- If you cancel during interactive staging, some commits may be incomplete
- Use `gt squash` if you need to undo a split

## See Also

- [gt squash](./squash.md) - Combine commits into one
- [gt fold](./fold.md) - Fold changes into an existing commit
- [gt modify](./modify.md) - Amend the current commit
