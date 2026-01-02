# gt land

Merge approved pull requests.

## Usage

```bash
gt land [OPTIONS] [branch]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[branch]` | Branch to land (default: current branch) |

## Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Show what would happen without making changes |
| `--all` | Land all approved PRs in the stack |

## Examples

### Land Current Branch

```bash
gt land
```

### Land Specific Branch

```bash
gt land feature/auth-models
```

### Land All Approved

```bash
gt land --all
```

### Preview First

```bash
gt land --dry-run
```

## Behavior

1. Checks PR status (must be approved and passing CI)
2. Merges the PR using the provider's API
3. Updates local tracking
4. Optionally syncs to rebase remaining stack

## Requirements

Before landing, a PR must:

- Have required approvals
- Pass all required status checks
- Have no merge conflicts

## Merge Strategy

The merge strategy depends on your repository settings:

- **Merge commit**: Creates a merge commit
- **Squash**: Squashes all commits into one
- **Rebase**: Rebases commits onto target

Stack respects your repository's configured merge strategy.

## After Landing

After landing a PR at the bottom of a stack:

```bash
gt land feature/step-1
gt sync  # Rebases remaining branches onto main
```

The remaining PRs will automatically update their base branches.

## Related Commands

- [`gt submit`](submit.md) - Create PRs
- [`gt sync`](sync.md) - Sync after landing
