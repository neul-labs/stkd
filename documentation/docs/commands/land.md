# gt land

Merge approved pull requests into the trunk branch.

Landing is the final step in the stacked diff workflow. It merges a PR via the provider API (GitHub or GitLab), deletes the local branch, and updates dependent branches.

## Usage

```bash
gt land [OPTIONS] [BRANCH]
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[branch]` | Branch to land (default: current branch) |

## Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Show what would happen without making changes |
| `--all` | Land all approved PRs in the stack |
| `--stack <name>` | Land all approved PRs in a specific stack |
| `--method <method>` | Merge method: `merge`, `squash`, or `rebase` |
| `--delete-branch` | Delete the branch after landing (default: true) |
| `--no-delete-branch` | Keep the branch after landing |
| `--skip-ci` | Merge even if CI hasn't finished |
| `--force` | Land even if the PR isn't fully approved |

## Examples

### Land Current Branch

```bash
# You're on feature/auth-models
gt land
```

Merges the PR for the current branch and deletes the local branch.

### Land Specific Branch

```bash
gt land feature/auth-models
```

You don't need to be on the branch to land it.

### Land All Approved PRs

```bash
# Land every approved PR in the current stack
gt land --all
```

Stack processes them in dependency order (bottom to top), lands each one, and syncs between landings.

### Land a Specific Stack

```bash
# If you have multiple stacks, land just one
gt land --stack feature/payment-models
```

### Choose Merge Method

```bash
# Create a merge commit
gt land --method merge

# Squash into a single commit (default for most repos)
gt land --method squash

# Rebase commits onto trunk
gt land --method rebase
```

The default method respects your repository settings on GitHub/GitLab. You can override it per landing.

### Preview Landing

```bash
gt land --dry-run
```

Output:

```
Would land:
  feature/auth-models (PR #42, 2 approvals, CI passing)
  Delete local branch: feature/auth-models
  Restack feature/auth-api onto main
  Restack feature/auth-ui onto feature/auth-api
```

### Keep Branch After Landing

```bash
# Land but keep the branch (useful for reference)
gt land --no-delete-branch
```

The branch is merged but remains locally. Stack still removes it from tracking metadata.

### Skip CI Check

```bash
# Land even if CI is still running
# (Use with caution — only if you know the build will pass)
gt land --skip-ci
```

### Force Landing

```bash
# Land even if the PR doesn't have required approvals
# (Requires maintainer permissions on the provider)
gt land --force
```

## Behavior

### Step-by-Step

```
1. VALIDATE PR STATUS
   - Query provider for PR #N status
   - Check approvals count
   - Check CI status (unless --skip-ci)
   - Verify no merge conflicts

2. MERGE
   - Call provider API to merge PR #N
   - Use specified merge method (or repo default)
   - Wait for merge confirmation

3. CLEAN UP LOCAL
   - Delete local branch (unless --no-delete-branch)
   - Remove Stack metadata for the branch
   - Update parent relationships for children

4. SYNC CHILDREN
   - If children exist:
     Update their parent to the landed branch's parent
     Restack children onto the new parent

5. REPORT
   - Show merge commit SHA
   - List deleted branches
   - List restacked branches
```

### Landing Order

Stack enforces dependency order. Given:

```
main
 └── feature/a      PR #1
      └── feature/b PR #2
           └── feature/c PR #3
```

You can land in this order:
1. `feature/a` (PR #1) — bottom-most
2. `feature/b` (PR #2) — after #1 lands
3. `feature/c` (PR #3) — after #2 lands

You **cannot** land `feature/c` first because its base branch (`feature/b`) hasn't been merged yet.

### Landing via Provider API

Stack uses the provider API to merge:

**GitHub:**
```bash
# Equivalent API call:
curl -X PUT \
  -H "Authorization: token $TOKEN" \
  -H "Accept: application/vnd.github.v3+json" \
  https://api.github.com/repos/org/repo/pulls/42/merge \
  -d '{"merge_method":"squash"}'
```

**GitLab:**
```bash
# Equivalent API call:
curl -X PUT \
  -H "PRIVATE-TOKEN: $TOKEN" \
  https://gitlab.com/api/v4/projects/123/merge_requests/42/merge \
  -d '{"squash":true}'
```

## Merge Methods

### Squash (Default)

All commits in the branch are squashed into a single commit on trunk:

```
Before:
main: M1 ── M2
              \
               A1 ── A2 ── A3  (feature/a)

After squash:
main: M1 ── M2 ── S1

Where S1 contains all changes from A1, A2, A3
```

**Pros:** Clean history, one commit per PR
**Cons:** Loses individual commit history

### Merge Commit

A merge commit is created, preserving branch history:

```
Before:
main: M1 ── M2
              \
               A1 ── A2 ── A3  (feature/a)

After merge:
main: M1 ── M2 ── M3 (merge commit)
                  \   /
                   A1 ── A2 ── A3
```

**Pros:** Preserves full history, shows parallel work
**Cons:** More complex history, merge commits in trunk

### Rebase

Commits are rebased onto trunk and fast-forwarded:

```
Before:
main: M1 ── M2
              \
               A1 ── A2 ── A3  (feature/a)

After rebase:
main: M1 ── M2 ── A1' ── A2' ── A3'
```

**Pros:** Linear history, no merge commits
**Cons:** Rewrites commit SHAs, can be confusing

### Choosing a Method

| Scenario | Recommended Method |
|----------|-------------------|
| Small PR (1-3 commits) | Squash |
| Large feature with meaningful commits | Merge commit |
| Team prefers linear history | Rebase |
| Individual commits tell a story | Merge commit |
| "One logical change" per PR | Squash |

## Handling Failures

### Merge Conflict on Landing

If someone merged to trunk while you were preparing to land:

```bash
$ gt land feature/auth-models
Merging PR #42...
ERROR: Merge conflict detected on main.

# Sync to get latest main
gt sync

# If the landed branch now has conflicts:
gt restack
gt submit  # Update the PR with resolved state
# Re-approve if needed, then land again
gt land feature/auth-models
```

### CI Failure

```bash
$ gt land feature/auth-models
ERROR: CI checks failed for PR #42.

# Fix the issue
gt checkout feature/auth-models
# Make changes...
git add .
gt modify
gt submit  # Updates PR #42
# Wait for CI to pass, then land
gt land feature/auth-models
```

### Approval Required

```bash
$ gt land feature/auth-models
ERROR: PR #42 requires 2 approvals, has 1.

# Ping reviewers or wait
# Or use --force (if you have permissions)
gt land feature/auth-models --force
```

### Branch Protection

```bash
$ gt land feature/auth-models
ERROR: Branch protection prevents direct merge.

# Check repository settings
# You may need to use the web UI
# Or configure Stack to use a merge queue
```

## Landing Multiple PRs

### Sequential Landing

```bash
# Land the bottom PR
gt land feature/step-1

# Sync updates children
gt sync

# Land the next
gt land feature/step-2

# Sync again
gt sync

# Land the next
gt land feature/step-3
```

### Batch Landing

```bash
# If all PRs are approved, land them all at once
gt land --all
```

This lands them in order automatically, syncing between each landing.

### Landing with Gaps

If you want to land PR #1 and #3 but not #2:

```bash
# Land #1
gt land feature/step-1
gt sync

# Update #3 to target main directly
gt checkout feature/step-3
gt track feature/step-3 --parent main
gt restack
gt submit

# Now #3 targets main, not #2
gt land feature/step-3
```

## After Landing

### What Happens to Child Branches

After landing a parent:

```
Before:
main
 └── feature/a      PR #1
      └── feature/b PR #2
           └── feature/c PR #3

After landing feature/a:
main ───────────────── feature/a (merged)
 └── feature/b         PR #2 (now targets main)
      └── feature/c    PR #3 (now targets feature/b)
```

`feature/b` automatically retargets to `main` because its old base (`feature/a`) no longer exists.

### Cleaning Up

After landing all PRs in a stack:

```bash
# All branches are merged and deleted
gt log
# main (no active stacks)

# Clean up any stale remote branches
git fetch --prune origin
```

## Tips for Smooth Landing

1. **Land promptly**: Approved PRs that sit accumulate conflicts
2. **Sync before landing**: Ensure you have the latest trunk state
3. **Use `--dry-run` first**: Preview what will happen
4. **Check CI status**: Don't land failing PRs
5. **Communicate with team**: Let reviewers know you're about to land
6. **Land bottom-up**: Children can't land before parents
7. **Use squash for small PRs**: Keeps history clean
8. **Batch with `--all`**: When everything is approved, land it all

## Related Commands

- [`gt submit`](submit.md) - Create or update PRs
- [`gt sync`](sync.md) - Sync after landing
- [`gt restack`](restack.md) - Rebase branches
