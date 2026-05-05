# Performance & Large Repositories

Stack is designed to be fast, but large repositories with thousands of branches or millions of files can slow things down. This guide covers configuration and workflow adjustments to keep Stack responsive.

---

## Understanding Stack's Performance Profile

Stack spends most of its time in these areas:

| Operation | Bottleneck | Mitigation |
|-----------|-----------|------------|
| `gt sync` | Git fetch + rebase | Shallow fetch, prune branches |
| `gt restack` | Sequential rebases | `--current-only`, smaller stacks |
| `gt submit` | Provider API calls | Batch operations, reduce metadata |
| `gt log` | Loading branch graph | Lazy loading, cache |
| TUI startup | Repository scan | Fast path for common operations |

---

## Shallow Fetch Configuration

Fetching full history on every sync is unnecessary for most stacked diff work.

### Configure Shallow Fetch

```bash
# Only fetch recent history
git config --global fetch.depth 50

# Or configure per-repo
git config fetch.depth 50

# Stack respects this setting during gt sync
gt sync
# Fetches only the last 50 commits
```

### Unshallow When Needed

If you need full history for a specific operation:

```bash
# Temporarily fetch full history
git fetch --unshallow

# Or just fetch a specific deep range
git fetch --depth=500
```

---

## Pruning Old Branches

Branches accumulate over time and slow down Stack's graph operations.

### Automatic Pruning

Stack removes merged branches during sync:

```bash
gt sync
# Automatically deletes merged branches from local tracking
```

### Manual Cleanup

```bash
# Delete all merged branches
gt sync --prune

# Or use Git directly to prune remote tracking branches
git fetch --prune origin

# Then sync to clean up Stack metadata
gt sync
```

### Aggressive Cleanup

For repositories with hundreds of stale branches:

```bash
# List branches not merged to main
gt log --all | grep -E "(merged|closed)"

# Delete specific old branches
gt delete feature/old-branch-1
gt delete feature/old-branch-2

# Bulk delete merged branches older than 30 days
# (Requires scripting against Stack's metadata)
```

---

## Sync Interval Tuning

How often should you sync? It depends on team velocity.

### High-Velocity Teams (many merges per hour)

```bash
# Sync every 30 minutes
gt sync

# Or configure auto-sync in the TUI
# Press 'y' in TUI every so often
```

### Low-Velocity Teams (few merges per day)

```bash
# Sync once in the morning is usually enough
gt sync

# Sync again before landing
gt sync && gt land feature/x
```

### Avoid Sync Thrashing

Don't sync between every command:

```bash
# Bad: syncs three times
gt sync && gt create feature/a && gt sync && gt create feature/b && gt sync

# Good: sync once at the start
gt sync
gt create feature/a
gt create feature/b
```

---

## Working with Monorepos

Monorepos (large repositories with many projects) present unique challenges.

### Stack Scope

Stack operates on the entire repository, not individual projects. In a monorepo:

```
monorepo/
├── frontend/
├── backend/
├── shared/
└── infra/
```

Your stacks can span multiple directories:

```bash
# A stack touching both frontend and backend
gt create feature/auth
git add frontend/src/auth backend/src/auth
gt modify
gt create feature/auth-tests
git add frontend/tests backend/tests
gt modify
```

### Performance Tips for Monorepos

1. **Use sparse checkout** if you only work in one directory:

```bash
git sparse-checkout set frontend/
# Stack still works, but Git operations are faster
```

2. **Avoid `gt log --all` in huge repos**:

```bash
# Slow: loads every branch
gt log --all

# Faster: just current stack
gt log
```

3. **Split large changes across fewer branches**:

In a monorepo, a "small" change might still touch 20 files. That's okay — the key is logical coherence, not line count.

---

## Many Branches Performance

Having 50+ active branches can slow down Stack.

### Stack Size Limits

| Stack Size | Performance | Recommendation |
|------------|-------------|--------------|
| 1-5 branches | Fast | Ideal |
| 6-10 branches | Slightly slower | Acceptable |
| 11-20 branches | Noticeable lag | Consider landing or splitting |
| 20+ branches | Slow | Break into independent stacks |

### Splitting Large Stacks

```bash
# You have a 15-branch monster stack
gt log
# feature/foundation
#   └── feature/api
#       └── feature/ui
#           └── ... 12 more branches

# Land the bottom half to free up the stack
gt land feature/foundation
gt sync
gt land feature/api
gt sync

# Now the top branches are direct children of main
# and the remaining stack is smaller
```

### Parallel Stacks

Instead of one giant stack, use parallel stacks:

```bash
# Instead of:
# main → A → B → C → D → E → F (6 branches)

# Do:
# main → A → B (stack 1)
# main → C → D (stack 2)
# main → E → F (stack 3)

gt checkout main
gt create feature/part-1
gt create feature/part-2

gt checkout main
gt create feature/other-1
gt create feature/other-2
```

---

## Storage Cleanup

Stack's metadata is lightweight, but it accumulates over time.

### Metadata Location

```
.git/stkd/
├── state.json          # Operation state
├── branches/           # Branch metadata (~200 bytes per branch)
│   ├── feature_a.json
│   └── ...
```

### Cleaning Up Stale Metadata

```bash
# Remove metadata for deleted branches
gt sync --prune

# Or manually clean up
gt log --all
# Identify branches that no longer exist
gt untrack feature/deleted-branch
```

### Repository Size

If your `.git/` directory is growing:

```bash
# Run Git garbage collection
git gc

# Aggressive cleanup (slow)
git gc --aggressive

# Remove unreachable objects
git prune
```

---

## Provider API Performance

Submitting and syncing involve API calls to GitHub/GitLab.

### Rate Limiting

GitHub limits unauthenticated requests to 60/hour and authenticated to 5,000/hour. Stack batches API calls, but large stacks can still hit limits.

```bash
# Check rate limit status
# (Stack shows warnings when approaching limits)

# Use a personal access token with higher limits
gt auth login github --token
```

### Reducing API Calls

```bash
# Submit without fetching MR status (fewer API calls)
gt submit --no-fetch

# Sync without provider checks
gt sync --no-provider
```

### Caching Provider Data

Stack caches MR status locally:

```
.git/stkd/
├── cache/
│   ├── github/
│   │   └── mr_cache.json
```

Cache expires after 5 minutes. You can force refresh:

```bash
# Refresh all provider data
gt sync --refresh

# Or in TUI, press 'g'
```

---

## TUI Performance

The TUI can become sluggish in large repos.

### Startup Optimization

The TUI loads the full branch graph on startup. In repos with 100+ branches:

```bash
# Use CLI instead for quick operations
gt log  # Faster than TUI startup

# Or filter the view
gt log --stack feature/auth
```

### Runtime Optimization

In the TUI:
- Press `g` sparingly (it triggers provider API calls)
- Use `j`/`k` navigation instead of loading full stacks
- Exit TUI (`q`) and use CLI for bulk operations

---

## Benchmarks

Expected performance on a typical repository (10,000 commits, 50 branches):

| Command | Time |
|---------|------|
| `gt log` | < 100ms |
| `gt sync` | 1-3s |
| `gt restack` (5 branches) | 2-5s |
| `gt submit` (5 branches) | 3-8s |
| TUI startup | 500ms - 1s |

On very large repos (1M+ commits, 500+ branches):

| Command | Time |
|---------|------|
| `gt log` | 500ms - 1s |
| `gt sync` | 5-10s |
| `gt restack` (5 branches) | 5-15s |
| `gt submit` (5 branches) | 5-15s |
| TUI startup | 2-5s |

---

## Performance Checklist

- [ ] Configure shallow fetch (`fetch.depth = 50`)
- [ ] Run `gt sync --prune` weekly
- [ ] Keep stacks under 10 branches
- [ ] Use parallel stacks instead of one giant stack
- [ ] Run `git gc` monthly
- [ ] Use `gt sync --no-provider` when you don't need MR status
- [ ] Avoid `gt log --all` in repos with 100+ branches
- [ ] Exit TUI for bulk operations
- [ ] Use sparse checkout in monorepos if applicable
- [ ] Keep Stack updated (performance improvements ship regularly)
