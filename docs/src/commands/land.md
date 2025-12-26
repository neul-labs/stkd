# gt land

Merge a pull request and clean up the branch.

## Synopsis

```
gt land [OPTIONS]
```

## Description

Merges the current branch's PR on the remote and cleans up the local branch. Can also land an entire stack from bottom to top.

## Options

| Option | Description |
|--------|-------------|
| `--method <METHOD>` | Merge method: merge, squash, rebase, ff (default: squash) |
| `-s, --stack` | Land entire stack from bottom to top |
| `--delete-local` | Delete local branches after landing (default: true) |
| `--no-sync` | Don't sync after landing |
| `-y, --yes` | Don't confirm before landing |
| `--dry-run` | Show what would be done without executing |

## Examples

### Preview Landing (Dry Run)

```bash
# See what would happen without actually doing it
gt land --dry-run

# Output:
# Dry run - showing what would be done:
#
# Branches to land:
#   → Merge MR #42 for feature/auth
#
# Merge method: squash
# Provider: github
# Will delete local branches after landing
# Will sync and switch to main
#
# Run without --dry-run to execute
```

### Land Current Branch

```bash
# Merge the current branch's PR
gt land

# Output:
# Branches to land:
#   → feature/auth (MR #42)
#
# Merge method: squash
# Provider: github
# Proceed with landing? [y/N] y
#
# Landing feature/auth (PR #42)...
# ✓ Merged PR #42 for feature/auth
# Syncing with remote...
# Switching to main...
# ✓ Landing complete!
```

### Land Entire Stack

```bash
# Land from bottom to top
gt land --stack

# Output:
# Branches to land:
#   → feature/models (PR #42)
#   → feature/api (PR #43)
#   → feature/ui (PR #44)
#
# Merge method: squash
# Proceed with landing? [y/N] y
#
# Landing feature/models (PR #42)...
# ✓ Merged PR #42
# Landing feature/api (PR #43)...
# ✓ Merged PR #43
# Landing feature/ui (PR #44)...
# ✓ Merged PR #44
# ✓ Landing complete!
```

### Specify Merge Method

```bash
# Use rebase instead of squash
gt land --method rebase

# Use merge commit
gt land --method merge

# Use fast-forward (if possible)
gt land --method ff
```

### Skip Confirmation

```bash
# Land without prompting
gt land --yes
```

### Dry Run for Stack

```bash
# Preview landing an entire stack
gt land --stack --dry-run

# Output:
# Dry run - showing what would be done:
#
# Branches to land:
#   → Merge MR #42 for feature/models
#   → Merge MR #43 for feature/api
#   → Merge MR #44 for feature/ui
#
# Merge method: squash
# Provider: github
# Will delete local branches after landing
# Will sync and switch to main
#
# Run without --dry-run to execute
```

## Merge Methods

| Method | Description | Best For |
|--------|-------------|----------|
| `squash` | Combine all commits into one | Clean history |
| `merge` | Create a merge commit | Preserving commits |
| `rebase` | Rebase onto target | Linear history |
| `ff` | Fast-forward only | Already rebased branches |

## Requirements

Before landing:

- PR must exist
- PR must be approved (if required)
- CI checks must pass (if required)
- No merge conflicts

## Landing Order

When using `--stack`, branches are landed in order:

```text
Stack: main → A → B → C

Landing order:
1. A → main (PR closes, A deleted)
2. B → main (was targeting A, now targets main)
3. C → main (was targeting B, now targets main)
```

## After Landing

Stack automatically:

1. Syncs with remote
2. Updates trunk (checkout and pull)
3. Cleans up merged branches locally
4. Updates remaining branch relationships

## Handling Failures

If a merge fails:

```bash
gt land

# Output:
# Landing feature/auth (PR #42)...
# ✗ Failed to merge PR #42: Merge conflict

# The landing stops here. Remaining branches are not landed.
```

To recover:

1. Check the PR on GitHub/GitLab
2. Resolve conflicts there or locally
3. Re-submit: `gt submit`
4. Try landing again: `gt land`

## Related Commands

- [gt submit](./submit.md) - Push and create PRs
- [gt sync](./sync.md) - Sync with remote
