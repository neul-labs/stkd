# Common Workflows

This page covers common patterns and workflows when using Stack.

## Daily Development Flow

A typical day with Stack:

```bash
# Start your day: sync with remote
gt sync

# Check your current stack
gt log

# Continue working on your current branch
# ... make changes ...
git add . && git commit -m "Continue feature work"

# Push updates
gt submit
```

## Starting a New Feature

When beginning new work:

```bash
# Always start from an updated main
git checkout main
git pull

# Create your first branch
gt create feature/my-feature

# Make your changes
git add . && git commit -m "Initial implementation"

# Push and create PR when ready
gt submit
```

## Breaking Down a Large PR

If you have a large PR that should be split:

```bash
# Checkout your large branch
git checkout feature/large-change

# Track it with Stack
gt track

# Create a checkpoint at a logical stopping point
gt create feature/large-change-part-2

# The original becomes the base, continue on part 2
# ... more changes ...

# Submit both
gt submit --stack
```

## Updating a Branch Mid-Stack

When you need to change a branch that has dependent branches:

```bash
# Switch to the branch you need to update
git checkout feature/base-change

# Make your changes
git add . && git commit -m "Address review feedback"

# Push the update
gt submit

# Now update dependent branches
git checkout feature/top-of-stack
gt sync
```

The `sync` command will rebase all dependent branches onto your changes.

## Handling a Merged Base

When a PR in the middle of your stack gets merged:

```bash
# Sync will detect the merge and update
gt sync

# This will:
# 1. Fetch latest from origin
# 2. Detect that the base PR was merged
# 3. Delete the merged local branch
# 4. Rebase remaining branches onto main
```

## Inserting a Branch

Need to add a branch between existing ones:

```bash
# You have: main → feature/a → feature/b (current)
# You want: main → feature/a → feature/new → feature/b

# Go to where you want to insert
git checkout feature/a

# Create the new branch
gt create feature/new

# Make changes
git add . && git commit -m "Add new feature"

# Now restack feature/b onto feature/new
git checkout feature/b
# Update parent tracking, then restack
gt track --parent feature/new
gt restack
```

## Multiple Independent Stacks

Working on unrelated features:

```bash
# Stack 1: Auth feature
git checkout main
gt create feature/auth-models
gt create feature/auth-api

# Stack 2: Settings feature (start from main again)
git checkout main
gt create feature/settings-page
gt create feature/settings-api

# View all stacks
gt log --all
```

## Collaborative Stacks

When working with teammates on a stack:

```bash
# Teammate pushes their branch
git fetch origin

# Track their branch as part of your stack
git checkout origin/feature/teammate-change
git checkout -b feature/teammate-change
gt track

# Stack your work on top
gt create feature/my-continuation
```

## Quick Fixes

For small fixes that don't need stacking:

```bash
# Create directly from main
git checkout main
git pull
gt create fix/typo

# Make the fix
git add . && git commit -m "Fix typo in README"

# Submit immediately
gt submit

# Land when approved
gt land
```

## Abandoning a Stack

When you need to abandon work:

```bash
# Delete branches from top to bottom
git checkout main
git branch -D feature/top
git branch -D feature/middle
git branch -D feature/base

# Or untrack them from Stack
gt untrack feature/top
gt untrack feature/middle
gt untrack feature/base
```

## Emergency Hotfix

When you need to ship a fix immediately:

```bash
# Save your current work
git stash

# Go to main and create a hotfix branch
git checkout main
git pull
gt create hotfix/critical-bug

# Make the fix
git add . && git commit -m "Fix critical bug"

# Submit and fast-track
gt submit
gt land --method squash

# Return to your work
git checkout feature/my-feature
git stash pop
```

## Best Practices

1. **Keep branches small**: Aim for < 400 lines per PR
2. **Logical commits**: Each branch should have one purpose
3. **Sync frequently**: Run `gt sync` at least once a day
4. **Submit early**: Create draft PRs for feedback
5. **Land promptly**: Don't let merged PRs pile up
