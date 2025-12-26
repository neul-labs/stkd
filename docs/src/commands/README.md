# Command Reference

Stack provides a set of commands for managing stacked diffs. All commands use the `gt` prefix.

## Quick Reference

| Command | Description |
|---------|-------------|
| `gt create` | Create a new stacked branch |
| `gt track` | Track an existing branch |
| `gt submit` | Push and create/update PRs |
| `gt sync` | Sync with remote and restack |
| `gt land` | Merge PRs |
| `gt log` | View stack structure |
| `gt status` | Show current branch status |
| `gt up` / `gt down` | Navigate the stack |
| `gt squash` | Squash commits in current branch |
| `gt fold` | Fold staged changes into a commit |
| `gt split` | Split a commit into multiple commits |
| `gt auth` | Manage authentication |
| `gt completions` | Generate shell completions |

## Global Options

These options work with all commands:

```
-h, --help       Print help information
-V, --version    Print version information
-q, --quiet      Suppress non-essential output
--debug          Enable debug output
```

## Command Categories

### Branch Management

- [gt create](./create.md) - Create a new stacked branch
- [gt track](./track.md) - Track an existing branch

### Remote Operations

- [gt submit](./submit.md) - Push branches and create/update PRs
- [gt sync](./sync.md) - Sync with remote, restack branches
- [gt land](./land.md) - Merge PRs

### Stack Visualization

- [gt log](./log.md) - View stack structure
- [gt status](./status.md) - Show current branch details

### Navigation

- [gt up / gt down](./navigation.md) - Move between branches

### Commit Editing

- [gt squash](./squash.md) - Squash commits in current branch
- [gt fold](./fold.md) - Fold staged changes into a previous commit
- [gt split](./split.md) - Split a commit into multiple commits

### Configuration

- [gt auth](./auth.md) - Manage provider authentication
- [gt completions](./completions.md) - Generate shell completions

## Examples

### Create and Submit a Stack

```bash
# Create first branch
gt create feature/step-1
# ... make changes ...
git commit -am "Step 1"

# Stack another branch
gt create feature/step-2
# ... make changes ...
git commit -am "Step 2"

# Submit the whole stack with reviewers
gt submit --stack --reviewers alice,bob
```

### Daily Workflow

```bash
# Start of day: sync with remote
gt sync

# View your current stack
gt log

# Continue work
# ... make changes ...
git commit -am "Continue work"

# Push updates
gt submit
```

### Landing PRs

```bash
# Preview what would be landed
gt land --dry-run

# After PR approval, land it
gt land

# Or land the entire stack
gt land --stack
```

### Commit Operations

```bash
# Squash all commits in branch
gt squash --all

# Fold staged changes into HEAD
git add file.rs
gt fold

# Split current commit into 3
gt split -c 3
```
