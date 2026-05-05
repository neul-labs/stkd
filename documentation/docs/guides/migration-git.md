# Migrating from Vanilla Git

Switching to stacked diffs from a traditional Git workflow requires a small shift in how you think about branches and commits. This guide maps familiar Git patterns to their Stack equivalents so you can adopt the workflow confidently.

---

## The Mental Model Shift

### Traditional Git: Commits on a Branch

In standard Git, you create one branch and make many commits:

```
main ─────────────────────────────────
     \
      feature/big-feature
       ├── Add models
       ├── Add API
       ├── Add UI
       ├── Add tests
       └── Fix bug
```

The branch is the unit of work. The PR contains all commits.

### Stack: Branches as Logical Commits

Stack treats each logical change as its own branch:

```
main
 └── feature/models      PR #1 (one logical change)
      └── feature/api    PR #2 (one logical change)
           └── feature/ui  PR #3 (one logical change)
```

The branch is the commit. The stack is the unit of work.

| Mental Model | Traditional Git | Stack |
|--------------|---------------|-------|
| Unit of work | Branch with many commits | Stack of single-purpose branches |
| Create work | `git checkout -b` | `gt create` |
| Save progress | `git commit` | `gt modify` |
| Update history | `git rebase -i` | `gt restack` |
| Share work | `git push` | `gt submit` |
| Land work | `git merge` | `gt land` |

---

## Workflow Mappings

### Feature Branch Workflow

**Traditional:**

```bash
# Start

git checkout -b feature/user-auth

# Work
git add .
git commit -m "Add user model"
git add .
git commit -m "Add auth API"
git add .
git commit -m "Add login UI"

# Push and PR
git push -u origin feature/user-auth
# Open PR #1 (1,200 lines)

# Land
git checkout main
git merge feature/user-auth
```

**Stack equivalent:**

```bash
# Start
gt checkout main
gt create feature/user-models

# Work on models
git add .
gt modify

# Stack the next change
gt create feature/auth-api
# Work on API
git add .
gt modify

# Stack the next change
gt create feature/auth-ui
# Work on UI
git add .
gt modify

# Push and PR
gt submit
# Creates PR #1 (150 lines), PR #2 (200 lines), PR #3 (180 lines)

# Land
gt land feature/user-models  # PR #1
gt sync                      # Updates PR #2 and #3 automatically
gt land feature/auth-api     # PR #2
gt sync
gt land feature/auth-ui      # PR #3
```

---

### Gitflow Workflow

**Traditional Gitflow:**

```
main ─────────────────────────────────────
      \
       develop ───────────────────────────
              \
               feature/login ──────────── PR
```

All feature PRs target `develop`. Large features still produce large PRs.

**Stack equivalent:**

```
main ─────────────────────────────────────
      \
       develop ───────────────────────────
                \
                 feature/auth-models      PR #1
                      └── feature/auth-api PR #2
                           └── feature/auth-ui PR #3
```

Stack replaces feature branches within Gitflow. You still merge `develop` → `main` for releases, but the feature PRs are smaller and reviewable.

```bash
# Same Gitflow structure, but smaller PRs
gt checkout develop
gt create feature/auth-models
gt create feature/auth-api
gt create feature/auth-ui
gt submit  # All target develop as base
gt land    # Merge into develop
```

---

### Trunk-Based Development

**Traditional trunk-based:**

```
main ── A ── B ── C ── D ── E ── F
```

Developers commit directly to main (or very short-lived branches) and use feature flags to hide incomplete work.

**Stack + trunk-based:**

Stack complements trunk-based development by letting you keep small PRs while still working on dependent changes:

```
main ──────────────────────────────────
 └── feature/flag-auth                 PR #1 (adds feature flag)
      └── feature/auth-models            PR #2 (models behind flag)
           └── feature/auth-api           PR #3 (API behind flag)
```

Benefits:
- Each PR is small and reviewable
- You don't need feature flags for every intermediate change
- You can land incrementally instead of one giant merge

---

## Common Git Commands → Stack Commands

| Git | Stack | Notes |
|-----|-------|-------|
| `git checkout -b feature/x` | `gt create feature/x` | Creates and tracks the branch |
| `git commit -am "..."` | `gt modify` | Amends current branch, then restacks children |
| `git rebase -i main` | `gt restack` | Restacks entire stack automatically |
| `git push origin feature/x` | `gt submit` | Pushes and creates/updates PRs |
| `git pull --rebase origin main` | `gt sync` | Fetches, detects merged branches, restacks |
| `git merge feature/x` | `gt land feature/x` | Merges PR via provider API |
| `git branch -d feature/x` | `gt delete feature/x` | Deletes branch and Stack metadata |
| `git rebase --abort` | `gt abort` | Aborts current operation |
| `git rebase --continue` | `gt continue` | Continues after resolving conflicts |
| `git status` | `gt log` | Shows stack tree and branch states |

---

## When NOT to Use Stacked Diffs

Stacked diffs shine for multi-step features, but they aren't always the right choice:

### Skip the Stack When

**Emergency hotfixes:**
```bash
# One line fix, land immediately
git checkout -b fix/critical-bug
git commit -m "Fix null pointer"
git push && gh pr create && gh pr merge
# Or just: gt create fix/critical-bug && gt submit && gt land
```

A single branch is simpler. Stack adds no value here.

**Documentation-only changes:**
```bash
# README update doesn't need stacking
gt create docs/readme-fix
git add .
gt modify
gt submit
gt land
```

While Stack works fine, a simple `git checkout -b` + `git push` is perfectly adequate.

**Tiny one-line fixes:**
```bash
# Fix typo in comment — no need for a stack
git commit -m "Fix typo"
```

**Projects with strict merge-commit policies:**

If your organization requires merge commits for auditability and forbids rebasing, the stacked diff workflow (which relies heavily on rebasing) may not fit.

**Teams that don't do code review:**

If PRs are rubber-stamped or bypassed, the overhead of creating multiple PRs has no payoff.

---

## Adapting Existing Workflows

### From Feature Branches

If you already have a feature branch with multiple commits:

```bash
# Your existing branch
git log --oneline main..feature/big-feature
# a1b2c3d Add tests
# e4f5g6h Add UI
# i7j8k9l Add API
# m0n1o2p Add models

# Convert to a stack
gt checkout main
gt track feature/big-feature --parent main

# Now split it into branches (optional)
gt checkout feature/big-feature
gt create feature/models
gt checkout feature/big-feature
gt create feature/api
# ... etc
```

### From Long-Running Branches

If you maintain long-running integration branches:

```bash
# Instead of one integration branch:
# main ── integration ── feature-a, feature-b, feature-c

# Use parallel stacks:
# main
#  ├── feature/a
#  └── feature/b
#       └── feature/c

gt checkout main
gt create feature/a
gt checkout main
gt create feature/b
gt create feature/c
gt submit  # All three are independent PRs
```

### From Fork-and-PR (Open Source)

If you work in a fork:

```bash
# Stack works with forks too
gt checkout main
gt create feature/my-contribution
gt create feature/my-contribution-part-2
gt submit  # Creates PRs targeting upstream main
```

Make sure your Stack provider is configured for the upstream repository.

---

## Getting Comfortable

### Week 1: Learn the Commands

Replace one workflow at a time:
- Day 1: Use `gt create` instead of `git checkout -b`
- Day 2: Use `gt submit` instead of `git push`
- Day 3: Use `gt sync` instead of `git pull --rebase`
- Day 4: Try `gt restack` after someone merges to main
- Day 5: Land your first PR with `gt land`

### Week 2: Build a Real Stack

Work on a real feature with 3-5 stacked branches. Experience:
- How reviews come in on different PRs at different times
- How `gt sync` updates bases after merges
- How restacking keeps everything aligned

### Week 3: Team Habits

- Share your stack links with reviewers
- Learn to review others' stacks
- Set up CI to handle stacked PRs

---

## Frequently Asked Questions

**"Do I have to stop using regular Git?"**

No. Stack is a layer on top of Git. You can use `git` commands anytime. Stack tracks branch relationships in `.git/stkd/` but doesn't prevent normal Git operations.

**"What happens to my existing branches?"**

Nothing. Stack only tracks branches you explicitly create with `gt create` or track with `gt track`. Existing branches are ignored until you add them.

**"Can I mix stacked and non-stacked work?"**

Yes. You can have a stack of tracked branches alongside regular untracked branches. Stack only operates on tracked branches.

**"Does this work with my existing CI/CD?"**

Yes. Stack creates regular PRs that CI sees the same way as any other PR. The only difference is the base branch might be another branch instead of `main`.

---

## Tips for a Smooth Transition

1. **Start small**: Pick one feature to stack, not your entire workflow
2. **Keep using `git`**: You don't need to unlearn Git — Stack extends it
3. **Sync frequently**: `gt sync` is your friend; run it before starting work each day
4. **Submit early**: Open PRs before they're perfect — that's the point
5. **Communicate with reviewers**: Let them know a PR is part of a stack
6. **Don't over-stack**: 3-5 branches is a sweet spot; 15 branches becomes unwieldy
