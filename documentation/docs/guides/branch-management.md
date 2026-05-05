# Branch Management Strategies

Effective branch management is the key to successful stacked diffs. This guide covers naming conventions, sizing, organization, and cleanup strategies.

---

## Branch Naming Conventions

Clear names make stacks easier to understand at a glance.

### Recommended Patterns

```
# Feature work
feature/add-user-auth
feature/user-profile-page
feature/oauth-integration

# Bug fixes
fix/login-redirect-loop
fix/memory-leak-in-parser
fix/race-condition-in-cache

# Refactors
refactor/extract-auth-service
refactor/simplify-error-handling
refactor/replace-legacy-api

# Documentation
docs/api-endpoint-reference
docs/contributing-guide

# Experiments
exp/new-build-system
exp/react-server-components
```

### Naming Tips

**Good:**

```
feature/add-password-reset      # Descriptive, clear scope
feature/auth-middleware         # Identifies the component
fix/oauth-callback-error        # Problem + context
```

**Bad:**

```
feature/work                    # Too vague
fix-bug                       # No context
step-1                        # Not descriptive
my-branch                     # Personal, not team-friendly
```

### Prefixes

| Prefix | Use For | Example |
|--------|---------|---------|
| `feature/` | New functionality | `feature/add-search` |
| `fix/` | Bug fixes | `fix/null-pointer` |
| `refactor/` | Code restructuring | `refactor/split-monolith` |
| `docs/` | Documentation | `docs/api-reference` |
| `exp/` | Experiments | `exp/new-parser` |
| `chore/` | Maintenance | `chore/update-deps` |

---

## How Big Should a Branch Be?

The golden rule: each branch should be reviewable in **15-30 minutes**.

### Size Guidelines

| Metric | Target | Maximum |
|--------|--------|---------|
| Lines changed | < 200 | < 500 |
| Files touched | < 10 | < 20 |
| Review time | 15 min | 30 min |
| Commits | 1-3 | < 5 |

### Why Small Branches?

- **Faster reviews**: Reviewers can approve quickly
- **Fewer conflicts**: Less chance of overlapping changes
- **Easier to revert**: One small change is safer to undo
- **Clearer history**: Each branch has a single purpose
- **Parallel review**: PRs can be reviewed independently

### When to Split

Split a branch when:

```
# Too big:
feature/user-system
  + models, API, UI, tests, migrations, docs
  = 800 lines, 25 files

# Better:
feature/user-models       # 150 lines, 4 files
feature/user-api          # 200 lines, 5 files
feature/user-ui           # 180 lines, 6 files
feature/user-tests        # 120 lines, 4 files
feature/user-docs         # 80 lines, 3 files
```

### When to Keep Together

Keep changes together when:

- They're tightly coupled (e.g., model + migration)
- Separating them would break CI
- The total is already under 200 lines

```
# Keep together:
feature/add-graphql-schema
  + schema definition
  + resolver
  + types
  = 180 lines, 6 files

# These belong together because the resolver
# doesn't work without the schema.
```

---

## Organizing Stacks

### Dependent Stacks

Branches that build on each other:

```
main
 └── feature/auth-models
      └── feature/auth-api
           └── feature/auth-ui
```

Use for: Features that naturally decompose into layers.

### Parallel Stacks

Independent features that branch from trunk:

```
main
 ├── feature/auth-system
 │    └── feature/oauth
 └── feature/dashboard
      └── feature/dashboard-widgets
```

Use for: Multiple unrelated features worked on simultaneously.

### Mixed Stacks

Combining dependent and parallel branches:

```
main
 ├── feature/core-refactor
 │    └── feature/api-v2
 └── feature/bugfix-login
      └── feature/bugfix-logout
```

Use for: Complex features with both dependent and independent work.

---

## Creating Branches

### From Current Branch (Default)

```bash
# You're on feature/a
gt create feature/b
# feature/b is stacked on feature/a
```

### From Trunk

```bash
# Create a new independent stack
gt checkout main
gt create feature/new-thing
```

### With Templates

```bash
# List available templates
gt create --list-templates

# Use a template
gt create --template feature my-feature
# Creates: my-feature/models, my-feature/api, my-feature/tests
```

---

## Cleaning Up Old Branches

### Automatic Cleanup

`gt sync` removes merged branches automatically:

```bash
gt sync
# Checks which branches were merged
# Deletes local tracking for merged branches
```

### Manual Cleanup

```bash
# Delete a branch and its Stack tracking
gt delete feature/old-branch

# Delete without removing tracking (keep Stack metadata)
gt delete feature/old-branch --keep-tracking

# Untrack without deleting the Git branch
gt untrack feature/old-branch
```

### Pruning Remote Branches

```bash
# Fetch and prune deleted remote branches
git fetch --prune origin

# Then sync to clean up Stack metadata
gt sync
```

---

## Renaming Branches

```bash
# Rename current branch
gt rename feature/better-name

# Update PRs after rename
gt submit
```

Stack updates the branch name in its metadata and pushes the renamed branch to remote.

---

## Tracking Existing Branches

Add existing branches to Stack:

```bash
# Track a branch with auto-detected parent
gt track feature/existing-branch

# Track with explicit parent
gt track feature/existing-branch --parent main

# Track a branch that's part of a stack
gt track feature/api --parent feature/models
```

!!! tip "Migrating to Stack"
    If you have existing feature branches, use `gt track` to add them to Stack before starting new work.

---

## Branch Ownership

In team environments, establish clear ownership:

### Single Owner

```
Alice owns:
  feature/auth-models
  feature/auth-api
  feature/auth-ui

Bob owns:
  feature/dashboard-layout
  feature/dashboard-widgets
```

### Shared Ownership

```
Alice creates: feature/foundation
Bob adds:      feature/foundation-api
Alice adds:    feature/foundation-ui
```

When sharing branches:

1. Communicate before rebasing
2. Use `gt sync` before pushing
3. Be careful with force pushes

---

## Common Patterns

### The Layer Cake

```
main
 └── feature/data-layer
      └── feature/business-logic
           └── feature/presentation-layer
```

Each layer depends on the one below. Classic for backend-to-frontend features.

### The Fan-Out

```
main
 └── feature/core-refactor
      ├── feature/refactor-api
      ├── feature/refactor-ui
      └── feature/refactor-tests
```

One foundational change with multiple independent follow-ups.

### The Experiment

```
main
 └── exp/new-parser
      └── exp/new-parser-integration
           └── exp/new-parser-benchmarks
```

An experimental feature that might be abandoned. Use `exp/` prefix to signal this.

---

## Branch Hygiene Checklist

- [ ] Names are descriptive and follow conventions
- [ ] Each branch is under 200 lines changed
- [ ] Branches are deleted after merging (`gt sync`)
- [ ] Old branches are pruned (`git fetch --prune`)
- [ ] Ownership is clear in team settings
- [ ] Branches are submitted promptly (don't let them age)

---

## Anti-Patterns

### The Mega-Branch

```
# Bad: 1,500 lines, 40 files
feature/everything
```

**Problem**: Too big to review, too risky to merge.

**Fix**: Split into 5-8 smaller branches.

### The Zombie Stack

```
# Branches left open for weeks
main
 └── feature/old-thing      # 3 weeks old, abandoned
      └── feature/older-thing  # 5 weeks old, abandoned
```

**Problem**: Stale branches accumulate conflicts.

**Fix**: Land or delete promptly. Use `gt sync` to clean up.

### The Dependency Tangle

```
# Bad: Circular or unclear dependencies
main
 ├── feature/a
 │    └── feature/b
 │         └── feature/a   # Cycle!
```

**Problem**: Stack can't restack cycles.

**Fix**: Redesign dependencies. Every branch must have exactly one parent.

### The Orphan Branch

```
# Branch not tracked by Stack
git checkout -b feature/orphan
# Stack doesn't know about this branch
```

**Problem**: Stack won't restack or submit it.

**Fix**: Always use `gt create` or `gt track`.
