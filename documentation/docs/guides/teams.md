# Working with Teams

Best practices for using Stack in team environments.

## Shared Branch Ownership

When multiple people work on a stack:

### Handoff Pattern

```bash
# Alice creates the stack
gt create feature/alice-part
gt submit

# Bob continues from where Alice left off
git fetch origin
gt checkout feature/alice-part
gt create feature/bob-part
gt submit
```

### Collaborative Editing

If you need to edit someone else's branch:

```bash
# Communicate with the owner first!

# Checkout their branch
gt checkout feature/teammate-branch

# Make changes
gt modify

# Push (they'll need to pull)
gt submit
```

!!! warning "Coordinate Force Pushes"
    Stack uses force-push after rebasing. Coordinate with teammates to avoid overwriting each other's work.

## Code Review Best Practices

### As an Author

1. **Keep PRs small** - Easier to review and approve
2. **Write good descriptions** - Explain the "why"
3. **Respond quickly** - Keep momentum going
4. **Update the whole stack** - After changes, `gt restack && gt submit`

### As a Reviewer

1. **Review in order** - Start from the bottom of the stack
2. **Approve incrementally** - Land PRs as they're ready
3. **Request changes clearly** - Specify which branch needs work

## Team Workflows

### Feature Lead Pattern

One person owns the stack structure:

```
Lead creates:
  main
   └── feature/foundation
        └── feature/api
             └── feature/ui

Team members:
  - Work on assigned branches
  - Create sub-stacks if needed
  - Lead merges and restacks
```

### Mob Programming

Team works together on a stack:

```bash
# One person drives
gt create feature/mob-session
# ... pair/mob programming ...

# Handoff to next driver
git push
# Next person pulls and continues
```

### PR Train

Sequential review and landing:

```
Day 1: Submit stack of 5 PRs
Day 2: PR #1 approved, landed
        gt sync
Day 3: PR #2 approved, landed
        gt sync
Day 4: PR #3,4 approved, landed
        gt sync
Day 5: PR #5 approved, landed
        Done!
```

## Communication

### Slack/Chat Integration

Share stack status with your team:

```bash
# Get stack overview
gt log --long

# Copy PR links
gt submit  # Shows PR URLs
```

### PR Description Templates

Stack adds stack visualization automatically:

```markdown
## Stack

- #101 Add user models ✅
- #102 Add user API ← this PR
- #103 Add user UI

## Changes

This PR adds the API layer for user management...
```

## Conflict Resolution

When multiple people modify the same stack:

```bash
# Person A pushes changes
gt submit

# Person B (after A's push)
gt sync  # Fetches A's changes
# May need to resolve conflicts
git add <resolved>
gt continue
gt submit
```

## Tips

1. **Communicate** - Let teammates know when you're rebasing
2. **Pull before push** - `gt sync` before `gt submit`
3. **Small PRs** - Easier to review and less conflict-prone
4. **Clear ownership** - Know who owns each branch
