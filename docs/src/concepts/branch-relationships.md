# Branch Relationships

Stack tracks the relationships between branches to manage your stack effectively. Understanding these relationships helps you work more efficiently.

## Parent and Child Branches

Every tracked branch has exactly one **parent** and zero or more **children**:

```text
main (trunk)
 │
 └── feature/base (parent: main)
      │
      ├── feature/part-a (parent: feature/base)
      │    │
      │    └── feature/part-a2 (parent: feature/part-a)
      │
      └── feature/part-b (parent: feature/base)
```

### Parent

The parent is the branch your branch was created from. When you run:

```bash
gt create feature/new
```

Stack records the current branch as the parent of `feature/new`.

### Children

Children are branches that were created on top of your branch. A branch can have multiple children (a "fork" in the stack).

## The Trunk

The **trunk** is the root of all stacks. It's typically:

- `main`
- `master`
- Or configured in `.stack/config`

The trunk is never tracked by Stack itself - it's the foundation everything builds on.

## Viewing Relationships

### Current Stack

See branches related to your current branch:

```bash
gt log
```

Output:

```text
┌ ○ feature/base
│ ◉ feature/current [you are here]
└ ○ feature/child
```

### All Branches

See all tracked branches and their relationships:

```bash
gt log --all
```

Output:

```text
main
 ├── feature/auth-base (#1)
 │    └── feature/auth-api (#2)
 │
 └── feature/settings (#3)
      ├── feature/settings-ui (#4)
      └── feature/settings-api (#5)
```

### Branch Info

Get details about a specific branch:

```bash
gt info feature/auth-api
```

Output:

```text
Branch: feature/auth-api
Parent: feature/auth-base
Children: (none)
PR: #42
Status: submitted
Created: 2024-01-15 10:30
Updated: 2024-01-15 14:22
```

## How Stack Uses Relationships

### Creating PRs

When you submit a branch, Stack uses the parent to set the PR's base:

```text
feature/child → PR base: feature/parent
feature/parent → PR base: main
```

### Restacking

When a parent changes, Stack knows which children need updating:

```text
If feature/base changes:
  → feature/part-a needs restack
    → feature/part-a2 needs restack
  → feature/part-b needs restack
```

### Landing

Stack ensures PRs land in the correct order:

```text
1. Land feature/base (into main)
2. Land feature/part-a (into main, now that base is gone)
3. Land feature/part-a2 (into main)
4. Land feature/part-b (into main)
```

## Changing Relationships

### Moving a Branch

Change a branch's parent:

```bash
# Move current branch onto a new parent
gt track --parent new-parent
gt restack
```

Before:

```text
main → A → B (current)
```

After:

```text
main → A
     → new-parent → B (current)
```

### Inserting a Branch

Insert a branch between existing ones:

```bash
# Starting with: main → A → C
git checkout A
gt create B

# Now: main → A → B
# But C still points to A

# Move C onto B
git checkout C
gt track --parent B
gt restack

# Now: main → A → B → C
```

### Orphaning a Branch

Remove a branch from the stack (but keep it):

```bash
gt untrack feature/experimental
```

The branch still exists but Stack no longer manages it.

## Relationship Rules

1. **Single Parent**: Each branch has exactly one parent
2. **No Cycles**: A branch cannot be its own ancestor
3. **Trunk is Root**: All branches eventually trace back to trunk
4. **Consistent State**: Children always based on parent's HEAD

## Troubleshooting

### "Branch has no parent"

This happens when:
- Branch was created outside Stack
- Parent was deleted without updating children

Fix:

```bash
# Track with explicit parent
gt track --parent main
```

### "Relationship mismatch"

The tracked parent doesn't match git history:

```bash
# View current state
gt info

# Re-sync relationships
gt sync --fix-parents
```

### "Circular dependency detected"

This shouldn't happen, but if it does:

```bash
# Reset branch tracking
gt untrack feature/broken
gt track --parent main feature/broken
```
