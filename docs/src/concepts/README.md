# Core Concepts

Understanding the core concepts behind Stack will help you use it more effectively.

## What Problem Does Stack Solve?

Traditional Git workflows often lead to:

- **Massive PRs**: Hundreds or thousands of lines that are hard to review
- **Long review cycles**: Big PRs take days or weeks to review
- **Merge conflicts**: Long-lived branches diverge from main
- **Blocked development**: Waiting for one PR blocks all dependent work

Stack solves these problems by enabling **stacked diffs**: a workflow where large features are broken into small, incremental, dependent changes.

## Key Concepts

### Stacks

A **stack** is a chain of branches where each branch builds on the previous one:

```text
main
 └── feature/step-1  (adds foundation)
      └── feature/step-2  (adds feature A)
           └── feature/step-3  (adds feature B)
```

### Parent-Child Relationships

Every tracked branch has:

- **Parent**: The branch it was created from
- **Children**: Branches created on top of it

Stack maintains these relationships to:
- Create PRs with correct base branches
- Rebase dependent branches when parents change
- Visualize the stack structure

### Trunk

The **trunk** is your main branch (usually `main` or `master`). All stacks eventually merge into the trunk.

### Restacking

When a parent branch changes, dependent branches need to be updated. This process is called **restacking**:

```text
Before:                    After rebase:
main ─┐                    main (updated) ─┐
      └─ A ─┐                              └─ A' ─┐
            └─ B                                  └─ B'
```

### Branch Status

Each branch has a status:

| Status | Meaning |
|--------|---------|
| `active` | Branch is being worked on |
| `submitted` | PR has been created |
| `merged` | PR has been merged |
| `closed` | PR was closed without merging |

## How Stack Tracks Branches

Stack stores metadata in `.git/stack/` including:

- Branch parent relationships
- PR numbers and URLs
- Branch status
- Creation and update timestamps

This metadata survives across Git operations and enables Stack's features.

## The Stack Workflow

1. **Create**: Start a new branch with `gt create`
2. **Develop**: Make commits as usual with Git
3. **Stack**: Create more branches on top with `gt create`
4. **Submit**: Push and create PRs with `gt submit --stack`
5. **Update**: Handle review feedback, restack if needed
6. **Land**: Merge PRs in order with `gt land`
7. **Sync**: Clean up and update with `gt sync`

## Learn More

- [What Are Stacked Diffs?](./stacked-diffs.md)
- [Restacking](./restacking.md)
- [Branch Relationships](./branch-relationships.md)
