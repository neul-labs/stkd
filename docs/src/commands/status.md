# gt status

Show detailed status of the current branch.

## Synopsis

```
gt status [OPTIONS]
```

## Description

Displays comprehensive information about the current branch including PR status, stack position, and working tree state.

## Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Show detailed PR information |
| `--fetch` | Fetch latest PR status from remote |

## Examples

### Basic Status

```bash
gt status

# Output:
# Current Branch
#   → feature/auth
#   Parent: main
#   Base: a1b2c3d
#
# Pull Request
#   → #42
#   https://github.com/owner/repo/pull/42
#
# Stack Position
#   Position: 2 of 3
#   Below: feature/models
#   Above: feature/ui
#
# Working Tree
#   ✓ Clean
```

### Verbose Status

```bash
gt status --verbose

# Output:
# Current Branch
#   → feature/auth
#   Parent: main
#   Base: a1b2c3d
#
# Pull Request
#   → #42
#   https://github.com/owner/repo/pull/42
#   State: Open
#   Mergeable: Yes
#   Merge state: clean
#
# Stack Position
#   Position: 2 of 3
#   Below: feature/models
#   Above: feature/ui
#
# Children
#   → feature/ui #44
#
# Working Tree
#   2 file(s) staged
#   1 file(s) modified
```

### Fetch Latest Status

```bash
gt status --fetch

# Fetches current PR state from GitHub/GitLab
```

## Output Sections

### Current Branch

Shows basic branch information:

- Branch name
- Parent branch
- Base commit (parent's HEAD when created)

### Pull Request

Shows PR information if one exists:

- PR number
- URL
- State (with `--verbose`)
- Merge status (with `--verbose`)

If no PR exists:

```
Pull Request
  ! No PR created
→ Run 'gt submit' to create a PR
```

### Stack Position

Shows where you are in the stack:

- Your position (1-indexed)
- Total branches in stack
- Branch above/below

### Children

If the branch has children, they're listed:

```
Children
  → feature/child-1 #45
  → feature/child-2 #46
```

### Working Tree

Shows Git working tree status:

- Clean (no changes)
- Staged files count
- Modified files count
- Untracked files count

## Untracked Branches

If you're on an untracked branch:

```bash
gt status

# Output:
# Current Branch
#   → some-branch
#   ! Not tracked by Stack
# → Run 'gt track' to start tracking 'some-branch'
```

## Related Commands

- [gt log](./log.md) - View stack structure
- [gt info](./navigation.md) - Info about specific branch
