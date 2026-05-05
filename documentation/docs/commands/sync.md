# gt sync

Sync with remote, update trunk, detect merged branches, and restack your local stacks.

`gt sync` is the most frequently used command in Stack. Run it multiple times per day to keep your stacks aligned with remote changes.

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
| `--no-delete` | Do not delete merged branches after syncing |
| `--no-restack` | Fetch and detect merges, but do not restack |
| `--no-provider` | Skip fetching MR/PR status from provider |
| `--force` | Force restack even if branches appear up-to-date |
| `--prune` | Prune merged remote branches before syncing |
| `--pull` | Pull the trunk branch into the working tree |

## Examples

### Basic Sync

```bash
gt sync
```

The default sync performs these steps:

1. Fetches from the remote
2. Checks if the trunk branch (e.g., `main`) has new commits
3. Identifies which of your tracked branches have been merged
4. Deletes merged branches from local tracking
5. Restacks remaining branches onto their updated parents
6. Fetches PR/MR status from the provider

### Preview Changes

```bash
gt sync --dry-run
```

Output:

```
Would sync:
  Fetch origin/main (3 new commits)
  Delete merged branches: feature/auth-models, feature/auth-api
  Restack feature/auth-ui onto main (1 commit)
  No conflicts expected
```

Use `--dry-run` when:
- You're unsure if a sync is needed
- You want to see which branches were merged
- You're in the middle of work and want to check before interrupting

### Auto-Sync Mode

Watch for changes and automatically sync:

```bash
gt sync --watch
```

With custom interval (poll every 30 seconds):

```bash
gt sync --watch --interval 30
```

!!! tip "When to Use Watch Mode"
    Watch mode is useful when:
    - You're waiting for a PR to be merged
    - You're in a review cycle and want your stack updated as soon as parents land
    - You're demoing Stack and want live updates

    Press `Ctrl+C` to stop watching.

### Preserve Merged Branches

By default, Stack deletes merged branches from local tracking. To keep them:

```bash
gt sync --no-delete
```

The branches remain in Git and Stack metadata, but Stack still detects that they were merged. Use this if you want to keep branch references for historical reasons.

### Sync Without Restacking

If you only want to fetch and check for merged branches without rebasing:

```bash
gt sync --no-restack
```

Useful when:
- You're in the middle of work and don't want your branch positions to change yet
- You want to manually control when rebasing happens
- You're about to go offline and want the latest metadata

### Skip Provider Checks

To speed up sync by skipping API calls to GitHub/GitLab:

```bash
gt sync --no-provider
```

This is faster but won't update PR status badges. Use when:
- You only care about Git state, not PR metadata
- You're on a slow connection
- You've hit API rate limits

### Force Restack

Restack even if Stack thinks everything is up-to-date:

```bash
gt sync --force
```

Use when:
- Stack's metadata got out of sync with Git state
- You made manual Git changes that Stack didn't detect
- A previous operation was interrupted and you want to ensure consistency

### Pull Trunk

After syncing, also check out and pull the trunk branch:

```bash
gt sync --pull
```

This updates your working tree to the latest `main` (or whatever your trunk is), which is useful if you want to start a new stack from the latest trunk.

## Behavior

### Step-by-Step Breakdown

```
1. FETCH
   git fetch origin

2. CHECK TRUNK
   git rev-parse origin/main
   If new commits: proceed to restack
   If no new commits: skip to step 4

3. DETECT MERGES (if trunk changed)
   For each tracked branch:
     Query provider: Is PR #N merged?
     Or check: Are branch commits in trunk?

4. DELETE MERGED
   gt delete feature/merged-branch

5. RESTACK
   In topological order (bottom to top):
     git rebase <updated-parent> <branch>

6. UPDATE METADATA
   Write new base commits to .git/stkd/branches/

7. FETCH PROVIDER STATUS (optional)
   Query GitHub/GitLab for PR states
```

### What Gets Synced

| Component | Synced | Notes |
|-----------|--------|-------|
| Trunk branch | Yes | Fetched from origin |
| Tracked branches | Yes | Rebases onto updated parents |
| PR/MR status | Yes | Unless `--no-provider` |
| Untracked branches | No | Stack ignores them |
| Local uncommitted changes | No | Sync fails if you have conflicts |
| Remote tags | No | Use `git fetch --tags` separately |

### Merge Detection

Stack detects merged branches in two ways:

1. **Provider query**: Ask GitHub/GitLab if the PR is merged
2. **Commit reachability**: Check if the branch's commits are reachable from trunk

Method 2 works even without a provider configured, but it can give false positives if someone cherry-picked your commits into trunk.

## Handling Conflicts

If conflicts occur during restack:

```bash
$ gt sync
Fetching origin...
Restacking feature/api onto main...
CONFLICT in src/models.rs

# Fix conflicts in your editor
$ vim src/models.rs

# Mark as resolved
$ git add src/models.rs

# Continue syncing
$ gt continue

# Or abort and restore state
$ gt abort
```

### Conflict Workflow

```
┌──────────────┐
│ gt sync      │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ CONFLICT     │
└──────┬───────┘
       │
       ▼
┌─────────────────┐
│ Fix conflicts   │
│ git add <files> │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
┌────────┐ ┌────────┐
│continue│ │ abort  │
│  resume│ │ restart│
└────────┘ └────────┘
```

## Watch Mode Deep Dive

### How Watch Mode Works

```bash
$ gt sync --watch --interval 30
Watching for changes... (press Ctrl+C to stop)
[14:32:05] No changes detected
[14:32:35] No changes detected
[14:33:05] Changes detected!
  Fetching origin...
  feature/auth-models merged, deleting...
  Restacking feature/auth-api onto main...
  Restacking feature/auth-ui onto feature/auth-api...
  All stacks synced.
[14:33:35] No changes detected
```

### Watch Mode with Notifications

On macOS, Stack can show desktop notifications when PRs are merged:

```bash
gt sync --watch --notify
```

### Filtering What to Watch

Watch only a specific stack:

```bash
gt sync --watch --stack feature/auth-models
```

## Tips for Effective Syncing

1. **Sync at the start of your day**: Catch up on overnight merges
2. **Sync before creating new branches**: Build on the latest trunk
3. **Sync before landing**: Ensure the PR hasn't been superseded
4. **Sync after long breaks**: Remote may have moved forward
5. **Use `--dry-run` to preview**: See what would change before it does
6. **Don't fear conflicts**: They're usually small if you sync frequently
7. **Watch mode during reviews**: Auto-update when parents get approved

## Troubleshooting

### "Nothing to sync" when you expected changes

```bash
# Check if the remote actually has new commits
git fetch origin
git log HEAD..origin/main --oneline

# If commits exist but Stack doesn't detect them:
gt sync --force
```

### "Cannot sync with uncommitted changes"

Stack refuses to sync if you have uncommitted changes that might conflict:

```bash
# Option 1: Commit or stash
git stash
gt sync
git stash pop

# Option 2: Use --no-restack (safer)
gt sync --no-restack
```

### Sync deleted a branch I still need

By default, Stack deletes merged branches. If you need to keep one:

```bash
# Prevent deletion for a specific branch
gt config sync.no-delete feature/important

# Or use --no-delete globally
gt sync --no-delete
```

### Watch mode is slow

If watch mode feels sluggish:

```bash
# Increase interval
gt sync --watch --interval 120

# Or skip provider checks
gt sync --watch --no-provider
```

## Related Commands

- [`gt restack`](restack.md) - Restack without fetching
- [`gt submit`](submit.md) - Push and create PRs
- [`gt land`](land.md) - Merge PRs
