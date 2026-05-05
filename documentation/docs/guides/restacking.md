# Restacking Deep Dive

Restacking is the core operation that keeps your stack in sync when parent branches change. Understanding how it works helps you use Stack confidently and resolve issues when they arise.

---

## What Is Restacking?

Restacking rebases all branches in a stack onto their updated parent branches. It ensures that every branch in your stack sits on top of the latest version of its parent.

### The Simple Case

Before restacking:

```
main (commit M1)
 └── feature/a (commit A1, based on M1)
      └── feature/b (commit B1, based on A1)
```

Someone pushes to `main`:

```
main (commit M1 → M2)
 └── feature/a (still based on M1)
      └── feature/b (still based on A1)
```

After `gt sync` (which includes restacking):

```
main (M2)
 └── feature/a (A1 rebased onto M2)
      └── feature/b (B1 rebased onto new A1)
```

Each branch is rebased onto its parent's new HEAD.

---

## When Restacking Happens

### Automatically

Restacking occurs automatically during these commands:

- **`gt sync`** — Fetches remote, then restacks if trunk or merged branches changed
- **`gt modify`** — Amends current branch, then restacks all dependent branches
- **`gt submit`** — Restacks before pushing if the stack is out of date

### Manually

You can trigger restacking explicitly:

```bash
# Restack all branches in the repository
gt restack

# Restack only current branch and its descendants
gt restack --current-only

# Show what would be done without doing it
gt restack --dry-run

# Force restack even if branches appear up-to-date
gt restack --force
```

---

## How Stack Tracks Dependencies

Stack stores a dependency graph in `.git/stkd/`:

```
.git/stkd/
├── state.json          # Operation state
├── branches/
│   ├── feature_a.json  # { "name": "feature/a", "parent": "main" }
│   ├── feature_b.json  # { "name": "feature/b", "parent": "feature/a" }
│   └── feature_c.json  # { "name": "feature/c", "parent": "feature/b" }
```

When restacking, Stack:

1. Loads the graph
2. Determines topological order (bottom to top)
3. For each branch, checks if its parent HEAD changed
4. If changed, rebases the branch onto the parent's new HEAD
5. Updates the branch's base commit reference
6. Repeats for all descendants

---

## The Restacking Algorithm

Stack uses a bottom-up approach:

```
Step 1: Identify roots (branches whose parent is trunk)
        └── feature/a (parent: main)

Step 2: Process each root
        Rebase feature/a onto latest main

Step 3: Process children
        Rebase feature/b onto new feature/a
        Rebase feature/c onto new feature/b

Step 4: Continue until all branches are processed
```

### Visual Example

Original state:

```
main: M1 ── M2 ── M3
              \
               A1 ── A2  (feature/a)
                      \
                       B1 ── B2  (feature/b)
                                \
                                 C1  (feature/c)
```

After `main` gets new commits:

```
main: M1 ── M2 ── M3 ── M4 ── M5
              \
               A1 ── A2          (feature/a, now behind)
                      \
                       B1 ── B2  (feature/b, now behind)
                                \
                                 C1  (feature/c, now behind)
```

After restacking:

```
main: M1 ── M2 ── M3 ── M4 ── M5
                                \
                                 A1' ── A2'  (feature/a rebased)
                                              \
                                               B1' ── B2'  (feature/b rebased)
                                                          \
                                                           C1'  (feature/c rebased)
```

Commits are rewritten (new SHAs) but the content is preserved.

---

## Conflicts During Restack

When two branches modify the same file, restacking may produce conflicts:

```bash
$ gt restack
Restacking feature/a onto main...
Restacking feature/b onto feature/a...
CONFLICT in src/auth.rs

# Fix the conflict
vim src/auth.rs

# Mark as resolved
git add src/auth.rs

# Continue restacking
gt continue

# Or abort and start over
gt abort
```

### Conflict Workflow

```
┌──────────────┐
│ gt restack   │
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

See [Resolving Conflicts](conflicts.md) for a full guide.

---

## Restacking After Landing a Middle PR

This is one of the most powerful features of stacked diffs. When a middle PR lands:

```
Before:
main
 └── feature/a      PR #1 (approved, ready to land)
      └── feature/b PR #2 (in review)
           └── feature/c PR #3 (in review)
```

After landing PR #1:

```bash
gt land feature/a
```

Stack:
1. Merges PR #1 into `main`
2. Deletes `feature/a` branch
3. Updates `feature/b` to target `main`
4. Rebases `feature/b` onto `main`
5. Repeats for `feature/c`

Result:

```
After:
main ───────────────── feature/a (merged)
 └── feature/b         PR #2 (now targets main)
      └── feature/c    PR #3 (now targets feature/b)
```

You don't need to manually rebase anything. `gt sync` handles it all.

---

## `gt restack --current-only`

Sometimes you only want to restack from the current branch down:

```bash
# You're on feature/c
gt restack --current-only
```

This only restacks `feature/c` and its descendants. Branches above `feature/c` (like `feature/a` and `feature/b`) are not touched.

**Use case**: You modified `feature/c` and want to update it without disturbing `feature/a` and `feature/b`.

---

## `gt restack --dry-run`

Preview what restacking would do without making changes:

```bash
$ gt restack --dry-run
Would restack:
  feature/a onto main (2 commits)
  feature/b onto feature/a (2 commits)
  feature/c onto feature/b (1 commit)
```

This is useful when:
- You're unsure if restacking is needed
- You want to check for potential conflicts
- You're learning how Stack works

---

## `gt restack --force`

Force restack even if branches appear up-to-date:

```bash
gt restack --force
```

**Use case**: Stack's metadata got out of sync with Git state, or you made manual Git changes that Stack didn't detect.

---

## Common Restacking Scenarios

### Scenario 1: Trunk Moved Forward

Your team merged other PRs to `main` while you were working:

```bash
gt sync
# Fetches main, detects it's ahead
# Restacks all your branches onto new main
```

### Scenario 2: You Amended a Parent Branch

You realized `feature/a` needs a fix:

```bash
gt checkout feature/a
# Make changes...
git add .
gt modify  # Amends feature/a
gt restack  # Rebases feature/b and feature/c onto amended feature/a
```

### Scenario 3: Rebasing a Single Branch

You want to move `feature/b` to sit directly on `main` instead of `feature/a`:

```bash
# Not directly supported by restack
# Instead, use track to change the parent:
gt checkout feature/b
gt track feature/b --parent main
gt restack
```

---

## Tips for Smooth Restacking

1. **Sync frequently**: `gt sync` multiple times per day keeps rebases small
2. **Keep branches focused**: Smaller branches = fewer conflicts
3. **Commit early**: Don't let uncommitted changes block restacking
4. **Use `--dry-run`**: Preview before restacking large stacks
5. **Restack after modifying**: Always run `gt restack` after `gt modify` on a parent branch
6. **Land promptly**: The longer a branch sits, the more likely conflicts become

---

## What Happens Under the Hood

When Stack restacks, it uses standard Git operations:

```bash
# For each branch in topological order:
git checkout feature/a
git rebase main                    # Rebase onto updated parent
git checkout feature/b
git rebase feature/a               # Rebase onto updated feature/a
git checkout feature/c
git rebase feature/b               # Rebase onto updated feature/b
```

Stack adds safety:
- File locking prevents concurrent operations
- State tracking allows `gt continue` and `gt abort`
- Automatic force-push with `--force-with-lease`
- Metadata updates after each successful rebase

---

## Restacking and Force Pushes

After restacking, branches have new commit SHAs. Stack pushes with `--force-with-lease`:

```bash
git push --force-with-lease origin feature/a
git push --force-with-lease origin feature/b
git push --force-with-lease origin feature/c
```

This is safe because `--force-with-lease` fails if someone else pushed to the branch since you last fetched. If this happens, `gt sync` fetches their changes first.

!!! warning "Coordinate with Teammates"
    If multiple people work on the same stack, coordinate rebases. Stack's `--force-with-lease` protects against accidental overwrites, but communication is still important.
