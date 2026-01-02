# gt sync

Sync with remote and restack all branches.

## Usage

```bash
gt sync [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Show what would happen without making changes |
| `--watch` | Watch for changes and sync automatically |
| `--interval <secs>` | Watch interval in seconds (default: 60) |

## Examples

### Basic Sync

```bash
gt sync
```

### Preview Changes

```bash
gt sync --dry-run
```

### Auto-Sync Mode

Watch for merged PRs and automatically restack:

```bash
gt sync --watch
```

With custom interval:

```bash
gt sync --watch --interval 30
```

## Behavior

1. Fetches from remote
2. Updates trunk branch
3. Identifies merged branches
4. Rebases remaining branches onto their updated parents
5. Reports any conflicts

## What Gets Synced

- Trunk branch is updated from remote
- Merged branches are detected and cleaned up
- Child branches are rebased onto updated parents

## Handling Conflicts

If conflicts occur during rebase:

```bash
# Resolve conflicts in your editor
git add <resolved-files>

# Continue the sync
gt continue

# Or abort
gt abort
```

## Watch Mode

Watch mode (`--watch`) continuously monitors for changes:

- Polls the remote at the specified interval
- Automatically syncs when changes are detected
- Useful when waiting for PR reviews

Press `Ctrl+C` to stop watching.

## Related Commands

- [`gt restack`](restack.md) - Restack without fetching
- [`gt submit`](submit.md) - Push and create PRs
- [`gt land`](land.md) - Merge PRs
