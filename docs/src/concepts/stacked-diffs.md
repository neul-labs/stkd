# What Are Stacked Diffs?

Stacked diffs are a software development workflow where large changes are broken into small, incremental, dependent pieces that can be reviewed and merged independently.

## The Traditional Approach

In a traditional workflow, you might develop a feature like this:

```text
main ──●──●──●──────────────────────●
                                    ↑
feature ──●──●──●──●──●──●──●──●──●─┘
          (2000 lines changed)
```

Problems with this approach:

- **Hard to review**: 2000 lines is overwhelming
- **Slow feedback**: Reviewers procrastinate on large PRs
- **All or nothing**: Can't merge part of the feature
- **Merge conflicts**: Long-lived branches diverge

## The Stacked Approach

With stacked diffs, the same feature becomes:

```text
main ──●──●──●──────────────────────●
               ↑        ↑        ↑  ↑
part-1 ────────●──●─────┘        │  │
               (300 lines)       │  │
                                 │  │
part-2 ──────────────────●──●────┘  │
                         (400 lines)│
                                    │
part-3 ────────────────────────●──●─┘
                               (300 lines)
```

Benefits:

- **Easy to review**: Each PR is focused and small
- **Fast feedback**: Reviewers can act quickly
- **Incremental merging**: Land pieces as they're approved
- **Reduced conflicts**: Shorter-lived branches

## Real-World Example

Consider adding user authentication to an app:

### Without Stacking

One massive PR containing:
- Database migrations
- User model
- Password hashing
- Session management
- API endpoints
- Frontend forms
- Tests for everything

Reviewer: "This is too much. I'll look at it later."

### With Stacking

**PR 1: Database & Model** (150 lines)
```
- Add users table migration
- Add User model
- Add password hashing
```
Reviewer: "Looks good, approved!"

**PR 2: Session Management** (200 lines)
```
- Add sessions table
- Add session middleware
- Add auth helpers
```
Reviewer: "Nice, just one comment on the middleware."

**PR 3: API Endpoints** (250 lines)
```
- Add login endpoint
- Add logout endpoint
- Add registration endpoint
```
Reviewer: "Approved!"

**PR 4: Frontend** (200 lines)
```
- Add login form
- Add registration form
- Add logout button
```
Reviewer: "Ship it!"

## Key Principles

### 1. Each Change Should Be Self-Contained

Even though changes build on each other, each PR should:
- Have a clear, single purpose
- Be independently testable
- Not break the build if landed alone

### 2. Order Matters

PRs must be merged in order (bottom to top). Stack enforces this by:
- Setting PR base branches correctly
- Preventing out-of-order merges

### 3. Keep Moving

Don't wait for one PR to merge before starting the next:
- Create the next branch immediately
- Start the next piece while waiting for review
- Parallelize your work

## When to Use Stacked Diffs

**Good candidates:**
- Multi-step features
- Refactoring in phases
- Large migrations
- API + UI changes
- Database + Code changes

**Not necessary for:**
- Single-file bug fixes
- Documentation updates
- Configuration changes
- Truly independent changes

## Common Patterns

### The Layer Cake

Each layer builds on the previous:

```text
main → model → api → ui
```

### The Feature Flag

Add infrastructure, then features:

```text
main → feature-flag → feature-a
                   └→ feature-b
```

### The Refactor-Then-Build

Clean up first, then add new code:

```text
main → cleanup → new-feature
```

## Summary

Stacked diffs are about:

- ✓ Breaking big changes into small pieces
- ✓ Maintaining dependencies between pieces
- ✓ Getting faster, better reviews
- ✓ Shipping incrementally
- ✓ Reducing merge conflicts

Stack makes this workflow practical and efficient.
