# Workflow Patterns

Common workflows and patterns for using Stack effectively.

## Basic Stacking Workflow

The most common workflow for a single feature:

```bash
# 1. Start fresh
gt checkout main
gt sync

# 2. Create your stack
gt create feature/models
# ... make changes, commit ...

gt create feature/api
# ... make changes, commit ...

gt create feature/tests
# ... make changes, commit ...

# 3. Submit PRs
gt submit

# 4. After reviews, make changes
gt checkout feature/models
# ... make fixes ...
gt modify
gt restack

# 5. Update PRs
gt submit

# 6. Land as PRs are approved
gt land feature/models
gt sync
# ... repeat ...
```

## Parallel Stacks

Work on multiple independent features:

```bash
# Stack 1: Auth feature
gt checkout main
gt create auth/models
gt create auth/api

# Stack 2: Dashboard feature (independent)
gt checkout main  # Start from main again
gt create dashboard/widgets
gt create dashboard/layout

# Submit both stacks
gt checkout auth/api
gt submit

gt checkout dashboard/layout
gt submit
```

## Dependent Stacks

When one feature depends on another:

```bash
# First feature
gt create feature/core
# ... implement core ...
gt submit

# Dependent feature
gt create feature/extension  # Builds on core
# ... implement extension ...
gt submit

# When core lands
gt sync
# extension now targets main automatically
```

## Review-Driven Workflow

Optimize for quick code review:

```bash
# Create small, focused branches
gt create step-1-types      # Just type definitions
gt create step-2-storage    # Storage implementation
gt create step-3-api        # API layer
gt create step-4-tests      # Tests

# Submit all at once
gt submit

# Reviewers can:
# - Approve step-1 immediately (small, obvious)
# - Request changes to step-3
# - Skip to step-4 to see test cases
```

## Iterative Development

Building features incrementally:

```bash
# Start with minimal implementation
gt create feature/v1
# ... basic implementation ...
gt submit

# Add enhancements (before v1 lands)
gt create feature/v1-enhancements
# ... additional features ...
gt submit

# v1 lands
gt sync
# v1-enhancements now targets main

# Continue building
gt create feature/v1-polish
# ...
```

## Hotfix Workflow

Emergency fixes while working on features:

```bash
# You're working on a feature
# feature/new-thing ← current

# Emergency! Need to fix production
gt checkout main
gt sync
gt create hotfix/critical-bug
# ... fix the bug ...
gt submit
gt land hotfix/critical-bug

# Back to your feature
gt checkout feature/new-thing
gt sync  # Pick up the hotfix
```

## Best Practices

### Keep Branches Small

- Aim for < 200 lines changed per branch
- Each branch should be reviewable in 15-30 minutes
- One logical change per branch

### Name Branches Clearly

```
feature/add-user-auth      # Good: descriptive
feature/step-1             # OK for related series
fix-bug                    # Bad: too vague
```

### Commit Early, Restack Often

```bash
# Make a change
gt modify

# Update downstream immediately
gt restack
```

### Use Draft PRs for WIP

```bash
gt submit --draft  # Mark as work-in-progress
# ... continue working ...
gt submit          # Ready for review
```

### Sync Before Starting

```bash
gt checkout main
gt sync
gt create feature/new-thing  # Start from fresh main
```
