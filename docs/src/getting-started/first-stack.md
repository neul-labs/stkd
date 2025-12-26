# Your First Stack

This tutorial walks you through creating your first stacked pull request workflow from start to finish.

## Scenario

You're building a user authentication feature that has three logical parts:

1. **Database models** - User table and migrations
2. **API endpoints** - Login, logout, registration
3. **Frontend** - Login form and session handling

Each part builds on the previous one, making it perfect for stacked diffs.

## Step 1: Set Up

Start from a clean main branch:

```bash
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Verify Stack is authenticated
gt auth status
# ✓ Authenticated as octocat
```

## Step 2: Create the First Branch

Create a branch for the database models:

```bash
gt create feature/auth-models
```

This creates a new branch based on `main` and tracks it in Stack.

Now make your changes:

```bash
# Add user model
cat > src/models/user.rs << 'EOF'
pub struct User {
    pub id: u64,
    pub email: String,
    pub password_hash: String,
}
EOF

git add src/models/user.rs
git commit -m "Add User model"
```

## Step 3: Stack the API Layer

With the models in place, create the API branch:

```bash
gt create feature/auth-api
```

This creates a new branch that's tracked as a child of `feature/auth-models`.

Add the API code:

```bash
# Add authentication endpoints
cat > src/api/auth.rs << 'EOF'
use crate::models::User;

pub async fn login(email: &str, password: &str) -> Result<User, AuthError> {
    // Implementation
}

pub async fn register(email: &str, password: &str) -> Result<User, AuthError> {
    // Implementation
}
EOF

git add src/api/auth.rs
git commit -m "Add authentication API endpoints"
```

## Step 4: Add the Frontend

Create the final layer:

```bash
gt create feature/auth-ui
```

Add the frontend code:

```bash
# Add login component
cat > src/components/Login.tsx << 'EOF'
export function Login() {
    const handleSubmit = async () => {
        await api.login(email, password);
    };
    return <form onSubmit={handleSubmit}>...</form>;
}
EOF

git add src/components/Login.tsx
git commit -m "Add login UI component"
```

## Step 5: View Your Stack

See the complete stack:

```bash
gt log
```

Output:

```text
┌ ○ feature/auth-models [active]
│ ○ feature/auth-api [active]
└ ◉ feature/auth-ui [active]
```

The `◉` indicates your current branch, `○` shows other branches in the stack.

## Step 6: Submit for Review

Push everything and create PRs:

```bash
gt submit --stack
```

Stack will:

1. Push `feature/auth-models` to origin
2. Create PR #1: `auth-models` → `main`
3. Push `feature/auth-api` to origin
4. Create PR #2: `auth-api` → `auth-models`
5. Push `feature/auth-ui` to origin
6. Create PR #3: `auth-ui` → `auth-api`

Each PR description includes a stack visualization:

```markdown
## Stack

```
  feature/auth-models (#1)
→ feature/auth-api (#2)
  feature/auth-ui (#3)
```
```

## Step 7: Make Changes After Review

Reviewer requests changes on PR #1 (auth-models). Update it:

```bash
# Switch to the models branch
gt down
gt down

# Or directly
git checkout feature/auth-models

# Make the requested changes
vim src/models/user.rs
git add src/models/user.rs
git commit -m "Add created_at timestamp to User"

# Push the update
gt submit
```

Now restack the dependent branches:

```bash
git checkout feature/auth-ui
gt sync
```

Stack rebases `auth-api` and `auth-ui` onto the updated `auth-models`.

## Step 8: Land the Stack

Once PRs are approved, land them in order:

```bash
# Land from the bottom of the stack
git checkout feature/auth-models
gt land

# The next branch becomes the new bottom
git checkout feature/auth-api
gt land

# Finally
git checkout feature/auth-ui
gt land
```

Or land the entire stack at once:

```bash
gt land --stack
```

## Summary

You've learned how to:

- ✓ Create a stack of dependent branches
- ✓ Submit all branches as linked PRs
- ✓ Update branches and restack dependents
- ✓ Land merged PRs and clean up

## Next Steps

- [Common Workflows](./workflows.md) - More patterns and tips
- [Handling Conflicts](../tutorials/handling-conflicts.md) - When things go wrong
