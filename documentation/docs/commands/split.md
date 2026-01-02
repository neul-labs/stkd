# gt split

Split the current commit into multiple commits.

## Usage

```bash
gt split
```

## Behavior

1. Resets HEAD while keeping changes staged
2. Opens interactive mode to create new commits
3. Prompts for commit message for each new commit

## Example Workflow

Starting with a commit that has too many changes:

```bash
# Start the split
gt split

# Now changes are staged but uncommitted
# Selectively stage and commit

# Commit part 1
git reset HEAD
git add src/models/
git commit -m "Add user models"

# Commit part 2
git add src/api/
git commit -m "Add API endpoints"

# Commit part 3
git add src/ui/
git commit -m "Add UI components"

# Finish
gt continue
```

## Use Cases

- Breaking up commits that are too large
- Separating unrelated changes
- Creating more granular history for review

## Tips

- Use `git add -p` for partial file staging
- Use `git status` to track what's left
- Run `gt restack` after to update child branches

## Related Commands

- [`gt squash`](squash.md) - Combine commits (opposite operation)
- [`gt fold`](fold.md) - Move changes between commits
- [`gt modify`](modify.md) - Amend single commit
