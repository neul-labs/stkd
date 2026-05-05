# How Stacked Diffs Work

Stacked diffs (also called stacked pull requests or stacked branches) are a workflow for breaking large changes into a series of small, dependent pull requests. This guide explains the concepts behind the workflow so you can use Stack effectively.

---

## The Problem with Large PRs

When you build a feature, you often touch many parts of the codebase:

```
┌─────────────────────────────────────────────┐
│  PR #42: "Add user authentication"           │
│                                             │
│  + 1,200 lines changed                      │
│  + 15 files touched                         │
│  + Database migrations                      │
│  + API endpoints                            │
│  + Frontend components                      │
│  + Tests                                    │
│                                             │
│  Review time: 2+ hours                      │
│  Approval rate: Low                         │
└─────────────────────────────────────────────┘
```

Large PRs are hard to review because:

- **Cognitive overload**: Reviewers must understand everything at once
- **Delayed feedback**: Authors wait longer for review
- **All-or-nothing merging**: One issue blocks the entire change
- **Difficult to revert**: A bug might be buried in a 1,200-line diff

---

## The Stacked Diffs Mental Model

Instead of one branch with many commits, stacked diffs treat each logical change as its own branch:

```
main
 └── feature/auth-models      PR #1: Add user models (150 lines)
      └── feature/auth-api    PR #2: Add auth API (200 lines)
           └── feature/auth-ui PR #3: Add login UI (180 lines)
```

Each branch depends on its parent. PR #2 includes all changes from PR #1, and PR #3 includes changes from both PR #1 and PR #2.

### Branches as Commits

In the stacked diffs model, you can think of branches the way you think of commits:

| Traditional Git | Stacked Diffs |
|----------------|---------------|
| One branch, many commits | Many branches, one commit each |
| `git commit -m "..."` | `gt create feature/step-N` |
| `git rebase -i` | `gt restack` |
| Merge one PR | Merge multiple PRs in order |

This doesn't mean you can't have multiple commits per branch — you can. But the default mental model is one logical change per branch.

---

## Dependency Graphs

Stack tracks the parent-child relationships between branches:

```
                    main
                   /    \
                  /      \
         feature/auth   feature/dashboard
            /    \              \
           /      \          feature/dash-api
    feature/auth-api      
         /
        /
  feature/auth-ui
```

This is a directed acyclic graph (DAG). Stack prevents cycles and ensures every branch has exactly one parent.

### What Stack Stores

For each branch, Stack records:

```json
{
  "name": "feature/auth-api",
  "parent": "feature/auth-models",
  "merge_request_id": 42,
  "merge_request_url": "https://github.com/org/repo/pull/42"
}
```

This metadata lives in `.git/stkd/` and is updated automatically as you work.

---

## Base Branch Management

Each PR in a stack targets its parent branch as the base:

| PR | Title | Base Branch |
|----|-------|-------------|
| #1 | Add user models | `main` |
| #2 | Add auth API | `feature/auth-models` |
| #3 | Add login UI | `feature/auth-api` |

This means:

- PR #1's diff shows only the model changes
- PR #2's diff shows only the API changes (because the base already includes models)
- PR #3's diff shows only the UI changes

### When a Parent Lands

After PR #1 is merged into `main`:

```
Before landing:
main
 └── feature/auth-models      PR #1
      └── feature/auth-api    PR #2 (base: feature/auth-models)
           └── feature/auth-ui PR #3 (base: feature/auth-api)

After landing PR #1:
main ───────────────────────── feature/auth-models (merged)
 └── feature/auth-api         PR #2 (base updated to main)
      └── feature/auth-ui     PR #3 (base updated to feature/auth-api)
```

Stack's `gt sync` handles this automatically:

1. Fetches latest `main`
2. Detects that `feature/auth-models` was merged
3. Deletes the local `feature/auth-models` branch
4. Rebases `feature/auth-api` onto `main`
5. Updates PR #2's base branch to `main`
6. Repeats for dependent branches

---

## How GitHub/GitLab Display Stacked PRs

### GitHub

GitHub shows the diff between the PR branch and its base branch. For stacked PRs, this means:

- PR #2's diff does NOT include PR #1's changes (because the base is `feature/auth-models`)
- This makes each PR independently reviewable
- After PR #1 lands, GitHub automatically updates the diff for PR #2

### GitLab

GitLab works similarly with merge requests. The "Changes" tab shows only the diff against the target branch.

---

## Merge Order Constraints

Stacked PRs must land in dependency order:

```
main
 └── A ── B ── C
```

**Valid order**: A → B → C  
**Invalid order**: C → B → A (C's base branch wouldn't exist)

Stack enforces this by:

1. Only allowing land on the bottom-most unmerged branch
2. Automatically updating base branches after each merge
3. Using `gt sync` to detect and handle merged branches

---

## Comparison with Other Workflows

### Feature Branch Workflow

```
Traditional:
main ────────────────────────
     \
      feature/big-feature ─── PR #1 (1,200 lines)
```

**Pros**: Simple, familiar  
**Cons**: Large PRs, slow reviews, all-or-nothing merging

### Gitflow

```
main ─────────────────────────────────
      \
       develop ───────────────────────
              \
               feature/login ─────── PR
```

**Pros**: Clear separation, release branches  
**Cons**: Complex branching model, merge commits, still has large feature PRs

### Trunk-Based Development

```
main ── A ── B ── C ── D ── E
```

**Pros**: Fast iteration, always deployable  
**Cons**: Requires feature flags, less reviewable

### Stacked Diffs

```
main
 └── A      PR #1 (reviewable)
      └── B PR #2 (reviewable)
           └── C PR #3 (reviewable)
```

**Pros**: Small PRs, parallel review, incremental landing  
**Cons**: Tooling required, learning curve, rebase complexity

---

## When to Use Stacked Diffs

### Good Fit

- Features that naturally decompose into steps
- Large refactors that touch multiple layers
- Experiments where you want early feedback
- Teams with strong code review culture

### Less Good Fit

- Emergency hotfixes (single change, land immediately)
- Documentation-only changes
- Tiny one-line fixes
- Teams that rarely do code review
- Projects with strict merge-commit policies

---

## Common Misconceptions

### "I need to finish PR #1 before starting PR #2"

No! That's the whole point. Start PR #2 while PR #1 is in review:

```bash
gt checkout feature/auth-models  # PR #1 is submitted, in review
gt create feature/auth-api       # Start PR #2 now
# ... work on API while waiting for PR #1 review ...
```

### "Stacked PRs are just rebasing"

Rebasing is the mechanism, but the workflow is the value. Stack handles rebasing automatically so you focus on the work, not the Git mechanics.

### "Each branch can only have one commit"

Not true. You can have multiple commits per branch:

```bash
gt create feature/models
# work...
git commit -m "Add user model"
# more work...
git commit -m "Add migration"
```

Stack tracks the branch, not individual commits. However, keeping branches atomic (one logical change) is the recommended practice.

### "Stacked diffs are only for experienced Git users"

Stack is designed to make stacked diffs accessible. The CLI commands are simple:

```bash
gt create    # Like git checkout -b, but tracked
gt submit    # Like git push, but creates PRs
gt sync      # Like git pull --rebase, but for the whole stack
```

---

## Key Principles

1. **Small changes win**: Aim for 200 lines or less per branch
2. **One purpose per branch**: Each PR should be reviewable in 15 minutes
3. **Submit early, submit often**: Get PRs open before they're perfect
4. **Sync frequently**: `gt sync` multiple times per day
5. **Land as you go**: Don't wait for the whole stack to be approved
6. **Communicate**: Let reviewers know about stack dependencies
