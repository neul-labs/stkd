# gt modify

Amend the current branch's commit with staged changes.

`gt modify` is the Stack equivalent of `git commit --amend`. It updates the current branch's commit and automatically restacks all dependent branches.

## Usage

```bash
gt modify [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `-m, --message <msg>` | New commit message |
| `--no-edit` | Keep existing commit message |
| `--all` | Stage all changes before amending |
| `--commit` | Create a new commit instead of amending |
| `--no-restack` | Amend but do not restack children |

## Examples

### Amend with Staged Changes

```bash
# Stage your changes
git add src/auth.rs

# Amend the current branch
gt modify
```

This amends the current branch's commit with the staged changes, then restacks all children.

### Amend and Change Message

```bash
gt modify -m "Add authentication middleware with JWT support"
```

### Amend Without Changing Message

```bash
git add forgotten-file.rs
gt modify --no-edit
```

### Stage All and Amend

```bash
# Stage all modified files and amend in one command
gt modify --all
```

Equivalent to:

```bash
git add --all
gt modify
```

### Create a New Commit Instead

```bash
# Add a new commit to the current branch instead of amending
gt modify --commit
```

This creates a new commit on the current branch. Children are still restacked onto the new tip.

### Amend Without Restacking

```bash
# Amend the current branch but don't restack children yet
gt modify --no-restack
```

Useful when:
- You plan to make more changes to the branch
- You want to control when restacking happens
- You're working offline and don't want to trigger expensive operations

## Behavior

### What `gt modify` Does

```
1. STAGE (if --all)
   git add --all

2. AMEND (or create new commit if --commit)
   git commit --amend (-m "message" if provided)
   OR
   git commit -m "message" (if --commit)

3. RESTACK CHILDREN (unless --no-restack)
   For each child branch:
     git rebase <updated-parent> <child>
     (preserving child commits)

4. UPDATE METADATA
   Record new HEAD commit for the branch
```

### Modify vs Regular Git Commit

| Operation | Git | Stack |
|-----------|-----|-------|
| Save changes | `git commit` | `gt modify` |
| New commit | `git commit -m "..."` | `gt modify --commit` |
| Amend last commit | `git commit --amend` | `gt modify` |
| After amend | Manual `git rebase` for children | Automatic `gt restack` |

### Visual Example

Before modifying `feature/a`:

```
main: M1 ── M2
              \
               A1 ── A2  (feature/a)
                      \
                       B1 ── B2  (feature/b)
                                \
                                 C1  (feature/c)
```

After `gt modify` on `feature/a`:

```
main: M1 ── M2
              \
               A1' ── A2'  (feature/a, amended)
                        \
                         B1' ── B2'  (feature/b, rebased)
                                  \
                                   C1'  (feature/c, rebased)
```

All children are automatically updated.

## Modify vs Fold

Stack has two ways to combine changes: `gt modify` and `gt fold`.

### `gt modify`

Amends the current branch with new changes:

```bash
gt checkout feature/auth-api
git add src/auth.rs
gt modify
```

- Adds changes to the current branch's commit
- Restacks children
- Use when: You're still working on the current branch

### `gt fold`

Combines the current branch into its parent:

```bash
gt checkout feature/auth-api
gt fold
```

- Merges `feature/auth-api` into `feature/auth-models`
- Deletes `feature/auth-api` branch
- Restacks children of `feature/auth-api` onto the new parent
- Use when: You realize the current branch shouldn't be separate

### When to Use Which

| Scenario | Use |
|----------|-----|
| Forgot to add a file to the current commit | `gt modify` |
| Need to fix a bug in the current commit | `gt modify` |
| Realized this branch is too small | `gt fold` |
| Want to combine two related changes | `gt fold` |
| Accidentally created an extra branch | `gt fold` |

## Handling Children During Modify

### Children with Conflicts

If amending the parent causes conflicts in children:

```bash
$ gt modify
Modified feature/auth-api.
Restacking feature/auth-ui onto feature/auth-api...
CONFLICT in src/auth.rs

# Fix conflicts
$ vim src/auth.rs
$ git add src/auth.rs
$ gt continue
```

### Modifying a Branch with Many Children

If the current branch has many descendants, restacking can take time:

```bash
# Temporarily skip restacking
gt modify --no-restack

# Make more changes...
git add .
gt modify --no-restack

# Restack everything at once when done
gt restack
```

### Modifying a Branch After Children Were Submitted

If children already have open PRs, amending the parent will update them:

```bash
gt checkout feature/auth-models
git add .
gt modify

# Children are restacked, but PRs aren't updated yet
gt submit  # Updates PRs for all affected branches
```

GitHub/GitLab automatically updates the diff for child PRs when the base branch changes.

## Tips for Effective Modification

1. **Modify frequently**: Small amendments are easier to review than large ones
2. **Use `--all` for quick fixes**: Save typing `git add .`
3. **Write good commit messages**: They become PR descriptions
4. **Don't modify merged branches**: Stack will warn you
5. **Check children before modifying**: If children have uncommitted changes, restack might conflict
6. **Use `--no-restack` when batching**: Amend multiple times, then restack once
7. **Fold don't modify**: If a branch is too small, fold it into its parent instead

## Troubleshooting Modify

### "No changes to amend"

```bash
$ gt modify
ERROR: No staged changes to amend.

# Stage changes first
git add .
gt modify

# Or use --all
gt modify --all
```

### "Cannot modify, branch has uncommitted changes"

```bash
# Commit or stash your working changes first
git stash
gt modify
git stash pop
```

### "Children have conflicts after modify"

This is expected if your amendment overlaps with child work:

```bash
# Fix each conflict
gt continue
# Repeat until all children are restacked
```

### "Branch is already merged"

```bash
$ gt modify
ERROR: Cannot modify merged branch feature/auth-models.

# Checkout a different branch
gt checkout feature/auth-api
gt modify
```

## Related Commands

- [`gt fold`](fold.md) - Fold current branch into parent
- [`gt squash`](squash.md) - Squash multiple commits in a branch
- [`gt restack`](restack.md) - Rebase branches
- [`gt split`](split.md) - Split a branch into multiple branches
