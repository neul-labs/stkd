# Restacking

Restacking is the process of updating dependent branches when a parent branch changes. It's essential for keeping your stack consistent and mergeable.

## Why Restacking?

When you update a branch in the middle of a stack, its children become out of sync:

```text
Before update:
main ──A──B──C
       │
parent ───D──E──F
              │
child ────────G──H──I

After updating parent:
main ──A──B──C
       │
parent ───D──E──F──F'  ← New commit added
              │
child ────────G──H──I  ← Still based on old F!
```

The child branch needs to be rebased onto the updated parent to include the new commit.

## How Stack Handles Restacking

Stack automatically detects when restacking is needed:

```bash
# After updating a parent branch
gt submit

# Then on a child branch
gt sync

# Stack will:
# 1. Detect parent has new commits
# 2. Rebase child onto updated parent
# 3. Push the updated child
```

## Manual Restacking

You can also trigger restacking manually:

```bash
# Restack current branch onto its parent
gt restack

# Restack all branches in the stack
gt sync --restack
```

## What Happens During Restacking

1. **Identify Base**: Stack finds where your branch diverged from its parent
2. **Collect Commits**: Gathers all commits unique to your branch
3. **Rebase**: Applies those commits onto the updated parent
4. **Update Children**: Recursively restacks any child branches

```text
Step by step:

1. Identify base (E):
   parent ───D──E──F──F'
                 │
   child ────────G──H──I

2. Collect commits (G, H, I)

3. Rebase onto F':
   parent ───D──E──F──F'
                     │
   child ────────────G'──H'──I'

4. If child has children, restack them too
```

## Handling Conflicts

Sometimes restacking produces conflicts:

```bash
gt sync

# Output:
# Restacking feature/child...
# CONFLICT (content): Merge conflict in src/main.rs
# Automatic rebase failed. Resolve conflicts and run 'gt continue'
```

To resolve:

```bash
# 1. Fix the conflicts in your editor
vim src/main.rs

# 2. Mark as resolved
git add src/main.rs

# 3. Continue the restack
gt continue

# Or abort if needed
gt abort
```

## Restack Strategies

### Eager Restacking

Restack frequently to avoid large conflicts:

```bash
# Do this often
gt sync
```

Pros:
- Smaller conflicts
- Always up to date
- Faster reviews

Cons:
- More frequent rebases
- More force-pushes

### Lazy Restacking

Wait until necessary:

```bash
# Only restack when landing
gt land
```

Pros:
- Fewer rebases
- Stable commit hashes
- Easier to track changes

Cons:
- Larger conflicts
- Outdated code

### Recommended: Middle Ground

```bash
# Restack when:
# 1. Starting work for the day
gt sync

# 2. Before requesting review
gt sync
gt submit

# 3. After parent is merged
gt sync
```

## Avoiding Restack Issues

### Keep Branches Small

Smaller branches = fewer conflicts:

```text
# Bad: One giant branch
main → huge-feature (50 files changed)

# Good: Many small branches
main → part-1 (5 files) → part-2 (5 files) → ...
```

### Restack Often

Don't let branches diverge:

```bash
# Do this daily
gt sync
```

### Watch for Overlapping Changes

If two branches modify the same code:

```bash
# Before creating a new branch
git diff parent-branch -- path/to/file

# If there's overlap, consider merging the parent first
```

## Restack vs Merge

Stack uses **rebase** (not merge) for restacking because:

- **Linear history**: Easier to understand
- **Clean PRs**: No merge commits cluttering diffs
- **Simpler conflicts**: One set of conflicts, not repeated

The tradeoff is that commit hashes change, requiring force-push. Stack handles this automatically.

## Summary

- Restacking updates child branches when parents change
- Use `gt sync` to restack your entire stack
- Resolve conflicts when they occur with `gt continue`
- Restack frequently to minimize conflicts
- Smaller branches make restacking easier
