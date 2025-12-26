# Handling Conflicts Tutorial

This tutorial covers how to resolve conflicts when they occur during stacking operations.

## When Conflicts Happen

Conflicts occur when:

1. **Restacking**: Parent branch changed in a way that conflicts with your changes
2. **Syncing**: Main branch has commits that conflict with your stack
3. **Updating**: Your changes conflict with teammate's changes

## Anatomy of a Conflict

When Stack encounters a conflict:

```bash
gt sync

# Output:
# Fetching from remote...
# ✓ Fetched from remote
# Updating main...
# ✓ Updated main
# Restacking branches...
# Restacking feature/my-change...
# CONFLICT (content): Merge conflict in src/main.rs
#
# Resolve conflicts and run 'gt continue'
# Or run 'gt abort' to cancel
```

## Step-by-Step Resolution

### Step 1: Identify Conflicts

```bash
git status

# Output:
# You are currently rebasing branch 'feature/my-change'.
#
# Unmerged paths:
#   (use "git add <file>..." to mark resolution)
#
#   both modified:   src/main.rs
```

### Step 2: Open the Conflicting File

```bash
vim src/main.rs
```

You'll see conflict markers:

```rust
fn process_data(input: &str) -> Result<Data> {
<<<<<<< HEAD
    // Parent's version
    let parsed = parse_strict(input)?;
=======
    // Your version
    let parsed = parse_lenient(input)?;
>>>>>>> feature/my-change
    Ok(Data::new(parsed))
}
```

### Step 3: Resolve the Conflict

Edit to combine or choose:

```rust
fn process_data(input: &str) -> Result<Data> {
    // Combined solution
    let parsed = if strict_mode() {
        parse_strict(input)?
    } else {
        parse_lenient(input)?
    };
    Ok(Data::new(parsed))
}
```

Remove all conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`).

### Step 4: Mark as Resolved

```bash
git add src/main.rs
```

### Step 5: Continue the Operation

```bash
gt continue

# Output:
# Continuing rebase...
# ✓ Restacked feature/my-change
# ✓ Restacked feature/next-change
# ✓ Sync complete
```

## Aborting

If you can't resolve or want to start over:

```bash
gt abort

# Output:
# Aborting operation...
# ✓ Restored to previous state
```

This reverts all changes from the failed operation.

## Conflict Types

### Content Conflicts

Same lines modified differently:

```
<<<<<<< HEAD
line from parent
=======
line from your branch
>>>>>>> your-branch
```

### Add/Delete Conflicts

File deleted in one branch, modified in another:

```bash
git status
# deleted by us:   src/old_file.rs
```

Resolution:

```bash
# Keep the file
git add src/old_file.rs

# Or confirm deletion
git rm src/old_file.rs
```

### Rename Conflicts

File renamed differently:

```bash
git status
# both renamed:   src/old.rs -> src/new.rs
#                 src/old.rs -> src/renamed.rs
```

Resolution:

```bash
# Choose one name
git add src/new.rs
git rm src/renamed.rs
```

## Prevention Strategies

### 1. Sync Frequently

```bash
# At least once daily
gt sync
```

Small, frequent rebases are easier than big ones.

### 2. Keep Branches Small

```bash
# Bad: Giant branch touching many files
gt create feature/everything

# Good: Small focused branches
gt create feature/models
gt create feature/api
gt create feature/ui
```

### 3. Coordinate with Team

Before starting work on shared areas:

```bash
# Sync first
gt sync

# Check who else is working here
git log --oneline main..origin/main
```

### 4. Use Different Files

When possible, structure code to minimize overlapping changes:

```
# Instead of one big file
src/all_features.rs

# Use separate files
src/features/
├── auth.rs
├── payments.rs
└── notifications.rs
```

## Complex Scenarios

### Multiple Conflicts in a Stack

When multiple branches conflict:

```bash
gt sync

# Output:
# CONFLICT in feature/branch-1
```

Resolve, then continue:

```bash
# Fix conflicts
git add .
gt continue

# Output:
# CONFLICT in feature/branch-2
```

Repeat for each branch:

```bash
# Fix conflicts
git add .
gt continue

# Output:
# ✓ Sync complete
```

### Conflict During Land

```bash
gt land

# Output:
# Merging PR #42...
# ✗ Merge conflict on GitHub
```

Resolution:
1. Open PR on GitHub
2. Use GitHub's conflict resolution or
3. Restack locally and force-push:

```bash
gt sync
gt submit
gt land
```

## Tools for Conflict Resolution

### Git Mergetool

```bash
git mergetool
```

Configure your preferred tool:

```bash
git config merge.tool vimdiff
# Or: vscode, meld, kdiff3
```

### VS Code

Open the file in VS Code for inline resolution with buttons for:
- Accept Current Change
- Accept Incoming Change
- Accept Both Changes

### Three-Way Merge

Understanding the versions:

- **HEAD**: The parent branch version
- **Yours**: Your branch version
- **Base**: Common ancestor (what both started from)

```bash
# See base version
git show :1:src/file.rs

# See HEAD version
git show :2:src/file.rs

# See your version
git show :3:src/file.rs
```
