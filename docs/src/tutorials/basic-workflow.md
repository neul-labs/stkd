# Basic Workflow Tutorial

This tutorial walks through a complete stacking workflow from start to finish.

## Scenario

You're fixing a bug that requires changes in three areas:

1. Fix the root cause in the data layer
2. Add a test for the fix
3. Update documentation

## Prerequisites

- Stack installed and authenticated
- A Git repository with a `main` branch
- Clean working tree

## Step 1: Start Fresh

```bash
# Ensure you're on main and up to date
git checkout main
git pull

# Verify Stack is ready
gt status
```

## Step 2: Create the First Branch

```bash
# Create branch for the core fix
gt create fix/data-validation
```

Make your changes:

```bash
# Edit the code
vim src/data.rs

# Commit
git add src/data.rs
git commit -m "Fix null pointer in data validation

Check for null before dereferencing user input.
Fixes #123"
```

## Step 3: Stack the Test

Create a second branch for tests:

```bash
gt create fix/data-validation-test
```

Add the test:

```bash
# Write the test
vim tests/data_test.rs

# Commit
git add tests/data_test.rs
git commit -m "Add test for data validation fix

Ensures null input is handled correctly."
```

## Step 4: Stack the Documentation

One more branch for docs:

```bash
gt create fix/data-validation-docs
```

Update documentation:

```bash
# Update docs
vim docs/api.md

# Commit
git add docs/api.md
git commit -m "Document null handling in data API"
```

## Step 5: View Your Stack

See what you've created:

```bash
gt log

# Output:
# ┌ ○ fix/data-validation [active]
# │ ○ fix/data-validation-test [active]
# └ ◉ fix/data-validation-docs [active]
```

## Step 6: Submit for Review

Push everything and create PRs:

```bash
gt submit --stack

# Output:
# Pushing 3 branch(es)...
# ✓ Pushed fix/data-validation
# ✓ Pushed fix/data-validation-test
# ✓ Pushed fix/data-validation-docs
#
# Creating PRs...
# ✓ Created PR #42 for fix/data-validation
#   https://github.com/owner/repo/pull/42
# ✓ Created PR #43 for fix/data-validation-test
#   https://github.com/owner/repo/pull/43
# ✓ Created PR #44 for fix/data-validation-docs
#   https://github.com/owner/repo/pull/44
```

## Step 7: Handle Review Feedback

Reviewer asks for a change in PR #42. Navigate there:

```bash
# Go to the first branch
gt down
gt down  # Or: git checkout fix/data-validation
```

Make the requested change:

```bash
vim src/data.rs
git add src/data.rs
git commit -m "Address review: add logging"
```

Push the update:

```bash
gt submit
```

## Step 8: Restack Dependent Branches

The dependent branches need to be updated:

```bash
# Go to the top of the stack
gt top  # Or: git checkout fix/data-validation-docs

# Sync to restack
gt sync

# Output:
# Restacking branches...
# ✓ Restacked fix/data-validation-test
# ✓ Restacked fix/data-validation-docs
```

Update the PRs:

```bash
gt submit --stack
```

## Step 9: Land the Stack

Once all PRs are approved, land them:

```bash
# Land from the bottom
git checkout fix/data-validation
gt land

# Land the test
git checkout fix/data-validation-test
gt land

# Land the docs
git checkout fix/data-validation-docs
gt land
```

Or land all at once:

```bash
gt land --stack
```

## Step 10: Clean Up

Stack automatically cleans up merged branches. Verify:

```bash
# Should show empty or your next stack
gt log --all

# Should be on main
git branch
# * main
```

## Summary

You've completed a full stacking workflow:

1. Created a stack of three related branches
2. Submitted them as linked PRs
3. Handled review feedback
4. Restacked after changes
5. Landed the merged PRs

## Next Steps

- [Multi-PR Features](./multi-pr-feature.md) - More complex scenarios
- [Handling Conflicts](./handling-conflicts.md) - When things go wrong
