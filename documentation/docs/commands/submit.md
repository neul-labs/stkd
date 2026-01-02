# gt submit

Create or update pull requests for the stack.

## Usage

```bash
gt submit [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Show what would happen without making changes |
| `--only <branch>` | Only submit specific branch |
| `--from <branch>` | Submit from branch to tip |
| `--to <branch>` | Submit from root to branch |
| `--draft` | Create PRs as drafts |
| `--no-draft` | Create PRs as ready for review |

## Examples

### Submit Entire Stack

```bash
gt submit
```

### Preview First

```bash
gt submit --dry-run
```

### Submit Partial Stack

Only the current branch:

```bash
gt submit --only feature/my-branch
```

From a specific branch to tip:

```bash
gt submit --from feature/step-2
```

From root to a specific branch:

```bash
gt submit --to feature/step-2
```

### Draft PRs

```bash
gt submit --draft
```

## Behavior

1. Pushes each branch to remote
2. Creates PRs for branches without existing PRs
3. Updates PRs for branches that already have PRs
4. Sets correct base branches for stacked PRs
5. Adds stack visualization to PR descriptions

## PR Descriptions

Stack automatically adds a visualization to PR descriptions:

```markdown
## Stack

- #123 Add user models ✅
- #124 Add authentication API ← this PR
- #125 Add login UI
```

## Force Push

Stack uses force-push (`--force-with-lease`) when necessary to update branches after rebasing. This is safe as long as you're the only one working on the branch.

## Related Commands

- [`gt sync`](sync.md) - Sync with remote
- [`gt land`](land.md) - Merge PRs
- [`gt restack`](restack.md) - Rebase branches
