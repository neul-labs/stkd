# Stack for Open Source Maintainers

Maintaining an open source project means reviewing contributions from developers with varying levels of familiarity with your workflow. Stack can help both maintainers and contributors manage complex changes.

---

## The Open Source Challenge

External contributors often submit large PRs because they:
- Don't know your preferred PR size
- Built the entire feature before submitting
- Aren't using a stacked diff workflow

As a maintainer, you're left with a 1,200-line PR that's hard to review and risky to merge.

Stack helps on both sides:
- **Contributors** can use Stack to break their work into reviewable pieces
- **Maintainers** can convert large PRs into stacks for easier review

---

## Encouraging Stacked Contributions

### Contributor Documentation

Add a `CONTRIBUTING.md` section about stacked diffs:

```markdown
## Submitting Changes

We welcome contributions of all sizes. For larger features,
please consider using stacked diffs to break your work into
smaller, reviewable PRs.

### Using Stack

1. Install Stack: `cargo install stkd-cli`
2. Initialize: `gt init`
3. Create your stack:
   ```bash
   gt create feature/your-change-step-1
   gt create feature/your-change-step-2
   ```
4. Submit: `gt submit`

Each PR should be reviewable in under 30 minutes.
```

### PR Templates for Stacks

Create a `.github/PULL_REQUEST_TEMPLATE.md` that stack-aware contributors can use:

```markdown
## Description

## Stack Context

Is this PR part of a stack?
- [ ] No, standalone change
- [ ] Yes, part of a stack (link to parent/child PRs below)

### Stack Links
- Parent PR: #(number)
- Child PR: #(number)

## Checklist
- [ ] Each PR in the stack is under 250 lines
- [ ] I've run `gt sync` before submitting
- [ ] Tests pass for this PR independently
```

### Automated Stack Detection

Use a GitHub Action to detect large PRs and suggest stacking:

```yaml
# .github/workflows/stack-suggestion.yml
name: Suggest Stacked Diffs
on:
  pull_request:
    types: [opened]

jobs:
  suggest:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/github-script@v7
        with:
          script: |
            const additions = context.payload.pull_request.additions;
            const deletions = context.payload.pull_request.deletions;
            const total = additions + deletions;

            if (total > 500) {
              github.rest.issues.createComment({
                issue_number: context.issue.number,
                owner: context.repo.owner,
                repo: context.repo.repo,
                body: `This PR changes ${total} lines. Consider breaking it into a stack of smaller PRs for easier review. See our [contributing guide](CONTRIBUTING.md#stacked-diffs).`
              });
            }
```

---

## Converting Contributor PRs into Stacks

When a contributor submits a large PR, you can split it into a stack for easier review.

### Manual Conversion

```bash
# 1. Fetch the contributor's branch
git fetch origin pull/123/head:pr-123

# 2. Explore the commits
git log --oneline main..pr-123
# a1b2c3d Add final integration
# e4f5g6h Add API endpoints
# i7j8k9l Add data models

# 3. Create a stack from the commits
gt checkout main
gt track pr-123 --parent main

# 4. Split into logical branches
gt checkout pr-123
gt create pr-123-models
gt checkout pr-123
gt create pr-123-api

# 5. Submit as a stack
gt submit --from pr-123-models
```

Now you have:
- PR #124: Models (small, reviewable)
- PR #125: API (small, reviewable, depends on #124)

You can review #124, merge it, then #125 automatically updates.

### Automated Conversion with Stack

Stack can automatically detect logical splits in a branch:

```bash
# Analyze a branch for logical splits
gt split --analyze pr-123
# Suggests split points based on:
# - File boundaries
# - Commit message patterns
# - Directory structure

# Auto-split into branches
gt split --auto pr-123
# Creates:
#   pr-123-1 (models)
#   pr-123-2 (API)
#   pr-123-3 (integration)
```

---

## Reviewing Stacked Contributions

### Review Order

Always review from the bottom of the stack up:

```
main
 └── feature/models      # Review this FIRST
      └── feature/api    # Then this
           └── feature/ui # Then this
```

Each PR builds on the one below it. Reviewing out of order causes confusion.

### Review Comments on Stacks

When you leave comments on a parent PR, the author may need to restack children:

```bash
# Contributor fixes your feedback
gt checkout feature/models
git add .
gt modify  # Amends models branch
gt restack  # Rebases children onto updated models
gt submit  # Updates all PRs
```

GitHub automatically updates diffs for child PRs when the base changes.

### Partial Approval

You can approve individual PRs in a stack without waiting for the whole thing:

```bash
# As a maintainer, you land approved PRs
gt land feature/models
# Contributor runs:
gt sync  # Models deleted, children restacked
gt land feature/api
```

---

## Landing External Stacks

### Permission Model

Contributors typically don't have merge permissions. Maintainers land PRs on their behalf:

```bash
# Maintainer reviews and lands
gt land feature/contributor-models
# This merges via GitHub API using maintainer credentials
```

### Handling Conflicts

If `main` moved forward while the contributor was working:

```bash
# Maintainer fetches latest
gt sync

# If restacking produces conflicts
gt restack
# Fix conflicts...
gt continue
gt submit  # Updates contributor's PRs with resolved state
```

The contributor sees the updated PRs and can continue working.

---

## Communication Templates

Use these templates when interacting with contributors using stacks.

### Suggesting a Stack

```markdown
Hi @contributor! Thanks for this PR. The changes look valuable,
but at 800 lines it's quite large to review thoroughly.

Would you be open to breaking this into a stack of smaller PRs?
Something like:
1. Data models
2. API endpoints
3. UI components

This makes review faster and reduces the risk of regressions.
Check out our [contributing guide](CONTRIBUTING.md) for how to
use Stack (`gt`) to manage this.
```

### Reviewing a Stack

```markdown
Great stack structure! Reviewed #42 (models) — approved with
minor comments. Once that's merged, I'll review #43 (API).

Please address the feedback on #42, then run:
```bash
gt modify
gt restack
gt submit
```
This will update all PRs automatically.
```

### After Landing a Parent

```markdown
Landed #42 (models). The branch has been merged and deleted.

PR #43 (API) automatically updated to target `main`. You can
continue building on it or land it next when ready.
```

---

## Working with Forks

Contributors work in forks, which adds complexity to stacking.

### Contributor Workflow (Fork)

```bash
# Contributor clones their fork
git clone https://github.com/contributor/project.git
cd project

# Add upstream remote
git remote add upstream https://github.com/original/project.git

# Configure Stack to use upstream as provider
gt init --upstream upstream

# Create stack
gt create feature/my-contribution
gt create feature/my-contribution/tests

# Submit creates PRs targeting upstream/main
gt submit
```

### Maintainer Workflow (Upstream)

```bash
# Maintainer sees PRs from contributor's fork
gt log --all
# Shows contributor stacks alongside local stacks

# Review and land as usual
gt land feature/my-contribution
```

---

## Tips for Open Source Maintainers

1. **Document the workflow**: Make `CONTRIBUTING.md` explicit about PR size preferences
2. **Be gentle with stack suggestions**: Not every contributor will want to learn a new tool
3. **Offer to split large PRs**: If a contributor can't use Stack, split it yourself
4. **Review promptly**: Stacked PRs lose value if they sit unreviewed
5. **Land as you go**: Don't wait for the whole stack to be approved
6. **Use Stack's TUI**: `gt tui` gives a great overview of all open stacks
7. **Set up CI for stacked PRs**: Ensure each PR passes independently
8. **Communicate the dependency chain**: Use PR descriptions to show stack context

---

## CI/CD for Stacked PRs

### GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI
on:
  pull_request:
    types: [opened, synchronize, reopened]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      # For stacked PRs, we need to fetch the base branch
      - name: Fetch base branch
        run: |
          git fetch origin ${{ github.base_ref }}:${{ github.base_ref }}

      - name: Run tests
        run: cargo test
```

### Handling Base Branches

Stacked PRs target other branches, not `main`. Ensure CI handles this:

```yaml
# Check out the correct base for stacked PRs
- uses: actions/checkout@v4
  with:
    ref: ${{ github.head_ref }}
- run: |
    git fetch origin ${{ github.base_ref }}
    git checkout ${{ github.base_ref }}
    git checkout ${{ github.head_ref }}
```

---

## Measuring Open Source Health

Track these metrics to see if stacked diffs improve your project:

| Metric | Before Stack | After Stack |
|--------|--------------|-------------|
| Average PR size | 500 lines | 150 lines |
| Average review time | 3 days | 1 day |
| Review rounds per PR | 2.5 | 1.5 |
| Time to merge (large features) | 2 weeks | 1 week |
| Contributor satisfaction | — | Survey quarterly |

---

## Getting Contributors Started

### Quick Start for Contributors

```bash
# 1. Install
cargo install stkd-cli

# 2. Configure (one-time)
gt auth login github

# 3. In your fork
gt init --upstream upstream

# 4. Create stack
gt create feature/my-fix

# 5. Work and submit
git add .
gt modify
gt submit
```

### One-Page Cheat Sheet

Create `STACK_CHEATSHEET.md` in your repo:

```markdown
# Stack Quick Reference

| Task | Command |
|------|---------|
| Create branch | `gt create feature/name` |
| Save work | `gt modify` |
| Submit PRs | `gt submit` |
| Update after review | `gt modify && gt restack && gt submit` |
| Land approved PR | `gt land feature/name` |
| See status | `gt log` |
```

This reduces the barrier to entry for contributors who are new to Stack.
