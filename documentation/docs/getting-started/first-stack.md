# Your First Stack

This tutorial walks you through creating your first stack of pull requests.

## Scenario

You're building a user authentication feature. Instead of one massive PR, you'll create three focused PRs:

1. **Database models** - User table and migrations
2. **API endpoints** - Login/logout routes
3. **UI components** - Login form

## Step 1: Start from Main

Make sure you're on your trunk branch with a clean working tree:

```bash
gt checkout main
gt sync
```

## Step 2: Create the First Branch

Create a branch for the database models:

```bash
gt create feature/auth-models
```

Now make your changes:

```bash
# Create your model files
echo "CREATE TABLE users..." > migrations/001_users.sql

# Commit the changes
git add .
git commit -m "Add user database models"
```

## Step 3: Stack the Second Branch

Without switching back to main, create the next branch on top:

```bash
gt create feature/auth-api
```

This branch now includes all changes from `feature/auth-models`.

Add the API code:

```bash
# Create API files
echo "pub fn login()..." > src/api/auth.rs

git add .
git commit -m "Add authentication API endpoints"
```

## Step 4: Stack the Third Branch

Create the UI branch:

```bash
gt create feature/auth-ui
```

Add the UI components:

```bash
echo "<form>...</form>" > src/components/login.html

git add .
git commit -m "Add login UI components"
```

## Step 5: View Your Stack

See the full stack structure:

```bash
gt log
```

Output:
```
  main
   └── feature/auth-models
        └── feature/auth-api
             └── feature/auth-ui ← you are here
```

## Step 6: Navigate the Stack

Move between branches easily:

```bash
# Go down to parent
gt down
# Now on feature/auth-api

# Go to the bottom
gt bottom
# Now on feature/auth-models

# Go to the top
gt top
# Now on feature/auth-ui
```

## Step 7: Submit All PRs

Submit the entire stack as pull requests:

```bash
gt submit
```

Stack creates three PRs:

| PR | Title | Base |
|----|-------|------|
| #1 | Add user database models | main |
| #2 | Add authentication API endpoints | feature/auth-models |
| #3 | Add login UI components | feature/auth-api |

!!! tip "PR Descriptions"
    Stack automatically adds a stack visualization to each PR description showing where it fits in the stack.

## Step 8: Handle Reviews

If reviewers request changes to the models PR:

```bash
# Go to that branch
gt checkout feature/auth-models

# Make changes
git add .
gt modify  # Amends the commit

# Restack dependent branches
gt restack

# Push updates
gt submit
```

## Step 9: Land the Stack

Once PRs are approved:

```bash
# Merge the first PR
gt land feature/auth-models

# Sync to update the stack
gt sync

# Now feature/auth-api targets main directly
# Merge it
gt land feature/auth-api

# And so on...
```

## Tips for Effective Stacks

1. **Keep branches focused** - Each branch should do one thing well
2. **Commit early, commit often** - Use `gt modify` to amend as needed
3. **Restack regularly** - Run `gt restack` after changes to parent branches
4. **Use descriptive names** - Branch names become part of PR titles

## What's Next?

- [Commands Reference](../commands/index.md) - Learn all available commands
- [Workflow Patterns](../guides/workflows.md) - Common patterns and best practices
- [Resolving Conflicts](../guides/conflicts.md) - Handle merge conflicts
