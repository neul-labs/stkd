# Team Workflow Tutorial

This tutorial covers patterns for using Stack effectively with a team.

## Shared Understanding

Before using Stack as a team, agree on:

1. Naming conventions for branches
2. PR size guidelines
3. Review expectations
4. Merge order protocols

## Naming Conventions

Establish a naming pattern:

```
<type>/<ticket>-<description>

Examples:
feature/PROJ-123-user-auth
fix/PROJ-456-null-check
refactor/PROJ-789-cleanup
```

For stacks, add a suffix:

```
feature/PROJ-123-user-auth
feature/PROJ-123-user-auth-api
feature/PROJ-123-user-auth-ui
```

## Handoff Pattern

When handing work to a teammate:

### Originator (Alice)

```bash
# Create the base work
gt create feature/base-work
# ... make changes ...
git commit -m "Base implementation"
gt submit

# Notify Bob: "Ready for you to build on top"
```

### Recipient (Bob)

```bash
# Get Alice's branch
git fetch origin
git checkout -b feature/next-work origin/feature/base-work
gt track --parent feature/base-work

# Build on top
# ... make changes ...
git commit -m "Next layer"
gt submit
```

### Continuing Together

Both can now work on their parts:

```
Alice: feature/base-work
Bob: feature/next-work
```

When Alice updates her branch:

```bash
# Alice pushes changes
gt submit

# Bob syncs to get Alice's changes
gt sync
gt submit
```

## Review Protocol

### Stack Owner Responsibilities

1. Keep PRs small and focused
2. Update PR descriptions with context
3. Respond to review comments promptly
4. Restack after changes

### Reviewer Responsibilities

1. Review in order (bottom to top)
2. Approve bottom PRs first
3. Note dependencies in comments
4. Don't block on later PRs

### Communication Template

In PR descriptions:

```markdown
## Stack Context

This PR is part of a stack. Please review in order:

1. #42 - Database models ← **Review first**
2. #43 - API endpoints ← This PR
3. #44 - Frontend UI

## Dependencies

This PR depends on #42 being merged first.

## Changes

[Description of this specific PR]
```

## Merge Order Enforcement

PRs in a stack must merge in order. Options:

### Manual Coordination

Team agrees to check before merging:

1. Is the parent PR merged?
2. Are there conflicts?
3. Have dependent PRs been restacked?

### GitHub Branch Protection

Configure rules:

```
Required status checks:
  - stack/parent-merged

Require branches to be up to date: Yes
```

### CI Integration

Add a check that verifies parent is merged:

```yaml
# .github/workflows/stack-check.yml
name: Stack Check
on: [pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - name: Check parent PR
        run: |
          # Verify base branch exists and is merged
          gh pr view "$BASE_BRANCH" --json state -q '.state'
```

## Parallel Work Patterns

### Feature Teams

Multiple people on the same feature:

```
main
 └── feature/auth-base (Alice)
      ├── feature/auth-api (Bob)
      │    └── feature/auth-api-tests (Charlie)
      └── feature/auth-ui (Diana)
```

Each person owns their branch and coordinates merges.

### Pair Programming

Working together on a stack:

```bash
# Alice creates the branch
gt create feature/pair-work
git commit -m "Initial work"
git push -u origin feature/pair-work

# Bob checks out
git fetch origin
git checkout feature/pair-work
gt track

# Both can now commit and push
git add . && git commit -m "More work"
git push
```

Use `--force-with-lease` for rebased branches:

```bash
# After rebasing
git push --force-with-lease
```

## Conflict Resolution Protocol

When conflicts arise:

1. **Owner resolves**: Branch owner fixes conflicts
2. **Communicate**: Notify affected teammates
3. **Restack chain**: All dependent branches need update

```bash
# Owner fixes and pushes
gt sync
gt submit

# Teammates sync their branches
gt sync
gt submit
```

## Emergency Procedures

### Hotfix During Stack Work

```bash
# Save current work
git stash

# Checkout main for hotfix
git checkout main
git pull
gt create hotfix/critical

# Make fix
git commit -m "Critical fix"
gt submit
gt land

# Return to stack work
git checkout feature/my-work
git stash pop
gt sync
```

### Abandoning a Shared Stack

If work needs to be abandoned:

1. Communicate with all contributors
2. Close all PRs
3. Delete remote branches
4. Each person cleans up locally:

```bash
gt sync  # Removes merged/closed branches
```

## Best Practices Summary

1. **Communicate stack plans** before starting
2. **Keep branches small** (< 400 lines)
3. **Sync frequently** (at least daily)
4. **Review in order** (bottom to top)
5. **Merge promptly** (don't let approved PRs sit)
6. **Coordinate on conflicts** (owner resolves)
7. **Document dependencies** in PR descriptions
