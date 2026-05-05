# gt restack

Rebase all tracked branches onto their updated parent branches.

Restacking is the core operation that keeps your stack in sync when parent branches change. Unlike `gt sync`, `gt restack` does not fetch from remote — it only rebases local branches.

## Usage

```bash
gt restack [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Show what would happen without making changes |
| `--current-only` | Restack only the current branch and its descendants |
| `--force` | Force restack even if branches appear up-to-date |
| `--onto <branch>` | Restack current branch onto a specific target |

## Examples

### Restack All Branches

```bash
gt restack
```

Restacks all tracked branches in topological order (bottom to top). Each branch is rebased onto the current HEAD of its parent.

### Restack Only Current Branch and Descendants

```bash
# You're on feature/auth-api
gt restack --current-only
```

This only restacks `feature/auth-api` and any branches stacked on top of it. Branches below `feature/auth-api` (like `feature/auth-models`) are not touched.

!!! tip "Use Case for --current-only"
    You're modifying `feature/auth-api` and want to update it without disturbing `feature/auth-models` above it. This is useful when:
    - The parent branch is under active review and you don't want to change its commits
    - You're experimenting with changes and don't want to affect the rest of the stack
    - The parent branch has conflicts that you're not ready to resolve

### Preview Changes

```bash
gt restack --dry-run
```

Output:

```
Would restack:
  feature/auth-models onto main (2 commits)
  feature/auth-api onto feature/auth-models (2 commits)
  feature/auth-ui onto feature/auth-api (1 commit)
```

### Force Restack

```bash
gt restack --force
```

Restacks even if Stack's metadata says everything is up-to-date. Use when:
- Stack's metadata got out of sync with Git state
- You made manual Git changes (e.g., `git commit --amend`) that Stack didn't detect
- A previous restack was interrupted

### Restack Onto a Different Parent

```bash
# Move feature/auth-api to sit directly on main
gt checkout feature/auth-api
gt restack --onto main
```

This rebases `feature/auth-api` onto `main` instead of its current parent (`feature/auth-models`). Stack updates the parent relationship in metadata automatically.

!!! warning "Changing Parent Relationships"
    Use `--onto` carefully. If `feature/auth-ui` depends on `feature/auth-api`, it will be restacked onto the new position of `feature/auth-api`.

## When to Restack

### Automatically

Restacking occurs automatically during these commands:

| Command | When Restacking Happens |
|---------|------------------------|
| `gt sync` | After fetching if trunk or merged branches changed |
| `gt modify` | After amending current branch |
| `gt submit` | Before pushing if the stack is out of date |
| `gt land` | After landing a PR, before syncing children |

### Manually

You should run `gt restack` manually when:

- You made manual Git changes to a parent branch
- You merged a branch outside of Stack (e.g., via GitHub web UI)
- You want to update children after fixing conflicts in a parent
- You're preparing to submit and want to ensure everything is aligned

## How Restacking Works

### The Algorithm

Stack uses a bottom-up approach:

```
Step 1: Identify roots (branches whose parent is trunk)
        └── feature/auth-models (parent: main)

Step 2: Process each root
        git checkout feature/auth-models
        git rebase main

Step 3: Process children
        git checkout feature/auth-api
        git rebase feature/auth-models
        git checkout feature/auth-ui
        git rebase feature/auth-api

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

## Handling Conflicts

When two branches modify the same file, restacking may produce conflicts:

```bash
$ gt restack
Restacking feature/a onto main...
Restacking feature/b onto feature/a...
CONFLICT in src/auth.rs

# Fix the conflict
$ vim src/auth.rs

# Mark as resolved
$ git add src/auth.rs

# Continue restacking
$ gt continue

# Or abort and restore previous state
$ gt abort
```

### Conflict Resolution Tips

1. **Identify the conflict source**: Is it a change in trunk or in a parent branch?
2. **Keep the parent's intent**: When in doubt, preserve the parent's version
3. **Fix once, propagate**: Resolving a conflict in a parent may eliminate it in children
4. **Use `--current-only`**: If only one branch has conflicts, restack it separately
5. **Test after resolving**: Run tests before `gt continue` to catch semantic conflicts

## Restacking After Landing a Middle PR

This is one of the most powerful features of stacked diffs:

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

## Performance Considerations

### Large Stacks

For stacks with 10+ branches, restacking can take several seconds:

```bash
# Time the operation
time gt restack
# gt restack  0.42s user 0.18s system 12% cpu 4.812 total

# For very large stacks, consider landing bottom branches first
gt land feature/step-1
gt sync
gt land feature/step-2
# etc.
```

### Rebasing Many Commits

If a branch has many commits (10+), rebase is slower:

```bash
# Consider squashing before restacking
gt checkout feature/big-branch
gt squash
# Now 10 commits become 1, restack is faster
gt restack
```

## Tips for Smooth Restacking

1. **Sync frequently**: `gt sync` multiple times per day keeps rebases small
2. **Keep branches focused**: Smaller branches = fewer conflicts
3. **Commit early**: Don't let uncommitted changes block restacking
4. **Use `--dry-run`**: Preview before restacking large stacks
5. **Restack after modifying**: Always run `gt restack` after `gt modify` on a parent branch
6. **Land promptly**: The longer a branch sits, the more likely conflicts become
7. **Use `--current-only` when experimenting**: Don't destabilize your whole stack

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
- File locking prevents concurrent Stack operations
- State tracking allows `gt continue` and `gt abort`
- Automatic force-push with `--force-with-lease` during submit
- Metadata updates after each successful rebase

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

## Related Commands

- [`gt sync`](sync.md) - Fetch and restack
- [`gt modify`](modify.md) - Amend commits
- [`gt continue`](../commands/index.md) - Continue after conflicts
- [`gt abort`](../commands/index.md) - Abort current operation
