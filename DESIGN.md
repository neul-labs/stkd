# Stack - Stacked Diffs for Git

A Graphite-compatible CLI for managing stacked pull requests on GitHub.

## Vision

Stack is an open-source alternative to Graphite that provides:
- **Stacked diffs workflow** - Break large changes into reviewable, dependent PRs
- **Graphite CLI compatibility** - Familiar `gt` commands for Graphite users
- **GitHub integration** - Seamless PR creation, sync, and merge
- **Offline-first** - Works without network, syncs when connected
- **VCS integration** - Optional integration with VCS for large assets + ML workflows

## Core Concepts

### 1. Stack
A **stack** is a linear chain of dependent branches, each building on the previous:

```
main
 └── feature/auth-base        (PR #1: Base authentication)
      └── feature/auth-oauth  (PR #2: Add OAuth, depends on #1)
           └── feature/auth-2fa (PR #3: Add 2FA, depends on #2)
```

### 2. Branch Tracking
Stack tracks metadata about each branch:
- Parent branch (dependency)
- Children branches (dependents)
- Associated PR number
- Commit range
- Review status

### 3. Restacking
When a parent branch changes, all descendants must be rebased. Stack automates this:

```
gt modify     # Amend current branch
gt restack    # Automatically rebase all descendants
```

### 4. Submitting
Create/update PRs for an entire stack with one command:

```
gt submit --stack   # Create PRs for current branch and all descendants
```

## Architecture

```
stack/
├── Cargo.toml              # Workspace manifest
├── crates/
│   ├── stkd-core/         # Core logic (no CLI, no network)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── branch.rs      # Branch operations
│   │   │   ├── stack.rs       # Stack management
│   │   │   ├── dag.rs         # Dependency graph
│   │   │   ├── rebase.rs      # Rebase/restack logic
│   │   │   ├── storage.rs     # Metadata persistence
│   │   │   └── config.rs      # Configuration
│   │   └── Cargo.toml
│   │
│   ├── stkd-github/       # GitHub API integration
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── api.rs         # GraphQL/REST client
│   │   │   ├── pr.rs          # PR operations
│   │   │   ├── auth.rs        # OAuth/token auth
│   │   │   └── sync.rs        # Remote sync
│   │   └── Cargo.toml
│   │
│   └── stkd-cli/          # CLI binary
│       ├── src/
│       │   ├── main.rs
│       │   ├── commands/
│       │   │   ├── mod.rs
│       │   │   ├── create.rs
│       │   │   ├── modify.rs
│       │   │   ├── submit.rs
│       │   │   ├── sync.rs
│       │   │   ├── log.rs
│       │   │   ├── nav.rs     # up, down, top, bottom
│       │   │   └── ...
│       │   └── output.rs      # Terminal formatting
│       └── Cargo.toml
│
├── docs/
│   ├── getting-started.md
│   ├── commands.md
│   └── vs-graphite.md
│
└── tests/
    └── integration/
```

## Data Model

### Branch Metadata (`.git/stack/branches/<branch-name>.json`)

```json
{
  "name": "feature/auth-oauth",
  "parent": "feature/auth-base",
  "children": ["feature/auth-2fa"],
  "pr_number": 42,
  "pr_url": "https://github.com/owner/repo/pull/42",
  "base_commit": "abc123",
  "head_commit": "def456",
  "created_at": "2025-01-15T10:00:00Z",
  "updated_at": "2025-01-15T12:00:00Z",
  "status": "open",
  "frozen": false
}
```

### Stack Configuration (`.git/stack/config.json`)

```json
{
  "version": 1,
  "trunk": "main",
  "remote": "origin",
  "github": {
    "owner": "dipankar",
    "repo": "myproject"
  },
  "submit": {
    "draft": false,
    "auto_title": true,
    "pr_template": true
  }
}
```

### Stack State (`.git/stack/state.json`)

```json
{
  "current_branch": "feature/auth-oauth",
  "pending_restack": [],
  "conflict_state": null,
  "last_sync": "2025-01-15T12:00:00Z"
}
```

## Command Reference

### Navigation
| Command | Description |
|---------|-------------|
| `gt up [n]` | Move up n branches in stack |
| `gt down [n]` | Move down n branches |
| `gt top` | Jump to stack tip |
| `gt bottom` | Jump to stack base |
| `gt checkout [branch]` | Switch to branch |

### Branch Management
| Command | Description |
|---------|-------------|
| `gt create <name>` | Create new branch on top of current |
| `gt modify [-m msg]` | Amend current branch |
| `gt delete <branch>` | Delete branch from stack |
| `gt rename <name>` | Rename current branch |
| `gt track <branch>` | Start tracking existing branch |
| `gt untrack` | Stop tracking current branch |
| `gt move --onto <target>` | Move branch to new parent |
| `gt fold` | Merge into parent branch |
| `gt split` | Split branch into multiple |

### Synchronization
| Command | Description |
|---------|-------------|
| `gt sync` | Sync with remote, restack |
| `gt restack` | Rebase stack on updated parents |
| `gt submit [--stack]` | Create/update PRs |
| `gt land` | Merge PR and cleanup |
| `gt get <branch>` | Fetch teammate's stack |

### Information
| Command | Description |
|---------|-------------|
| `gt log` | Show stack structure |
| `gt log short` | Compact stack view |
| `gt log long` | Full commit graph |
| `gt info` | Show current branch info |
| `gt status` | Show pending operations |

### Conflict Resolution
| Command | Description |
|---------|-------------|
| `gt continue` | Continue after conflict resolution |
| `gt abort` | Abort current operation |

### Configuration
| Command | Description |
|---------|-------------|
| `gt init` | Initialize stack in repo |
| `gt auth` | Authenticate with GitHub |
| `gt config` | View/edit configuration |

## Implementation Phases

### Phase 1: Core Stack Operations (2 weeks)
- [ ] Project setup (Cargo workspace, CI)
- [ ] Branch tracking metadata
- [ ] Create/delete/rename branches
- [ ] Navigation (up/down/top/bottom)
- [ ] `gt log` visualization
- [ ] Basic restack logic

### Phase 2: Local Workflow (2 weeks)
- [ ] `gt modify` (commit amending)
- [ ] `gt fold` (merge into parent)
- [ ] `gt move` (reparent branch)
- [ ] `gt split` (divide branch)
- [ ] Conflict resolution flow
- [ ] `gt undo` support

### Phase 3: GitHub Integration (2 weeks)
- [ ] GitHub authentication (OAuth + token)
- [ ] `gt submit` - create/update PRs
- [ ] `gt sync` - fetch remote state
- [ ] `gt land` - merge and cleanup
- [ ] PR description templates
- [ ] Stack visualization in PR body

### Phase 4: Advanced Features (2 weeks)
- [ ] `gt get` - fetch teammate's stack
- [ ] `gt absorb` - smart change absorption
- [ ] AI-powered PR titles/descriptions
- [ ] GitLab support
- [ ] VCS integration (intent sharing)

### Phase 5: Polish (1 week)
- [ ] Shell completions
- [ ] Comprehensive tests
- [ ] Documentation
- [ ] Performance optimization

## Key Algorithms

### Restack Algorithm

```
function restack(branch):
    parent = get_parent(branch)
    if parent != trunk:
        restack(parent)  # Ensure parent is current

    if needs_rebase(branch, parent):
        rebase(branch, onto=parent)

    for child in get_children(branch):
        restack(child)
```

### Stack Detection

```
function detect_stack(branch):
    stack = []
    current = branch

    # Walk up to trunk
    while current != trunk:
        stack.insert(0, current)
        current = get_parent(current)

    # Walk down to include descendants
    queue = get_children(branch)
    while queue:
        child = queue.pop(0)
        stack.append(child)
        queue.extend(get_children(child))

    return stack
```

### Submit Strategy

```
function submit_stack(branch, include_descendants):
    branches = [branch]
    if include_descendants:
        branches.extend(get_all_descendants(branch))

    for b in branches:
        if has_pr(b):
            update_pr(b)  # Force push, update description
        else:
            create_pr(b, base=get_parent(b))
```

## Integration with VCS

Stack can optionally integrate with VCS for enhanced workflows:

### Shared Intent Metadata
When both VCS and Stack are installed, commits can include:
- Agent attribution (who/what made the change)
- Change reasoning
- Risk assessment

### Asset-Aware Stacking
Stack can detect VCS-tracked assets and:
- Warn when stacking large asset changes
- Suggest splitting assets into separate stacks
- Coordinate with VCS policy gates

### Configuration
```toml
# .git/stack/config.toml
[vcs]
enabled = true
share_intent = true
respect_policy_gates = true
```

## Differences from Graphite

| Feature | Graphite | Stack |
|---------|----------|-------|
| Open source | No | Yes |
| Self-hosted | No | Yes |
| GitHub only | Yes | GitHub + GitLab (planned) |
| Web dashboard | Yes | No (CLI-only, for now) |
| AI features | Paid | Optional (BYO API key) |
| VCS integration | No | Yes |

## Technical Decisions

### Why Rust?
- Fast startup time (critical for CLI)
- Single binary distribution
- Memory safety
- Excellent Git library (git2)
- Consistent with VCS codebase

### Why Git Notes for Metadata?
Considered but rejected:
- Git notes are complex and not widely understood
- Risk of conflicts with VCS which also uses notes

Decision: Use `.git/stack/` directory
- Simple JSON files
- Easy to debug
- No conflict with other tools
- Gitignored by default

### Why GraphQL for GitHub?
- More efficient queries (get multiple resources at once)
- Better support for mutations
- Strongly typed

Fallback to REST API when needed.

## Success Metrics

1. **Adoption**
   - 100+ GitHub stars in first month
   - Used by 3+ teams in production

2. **Performance**
   - `gt log` < 50ms
   - `gt submit` < 2s per PR
   - `gt restack` handles 10+ branch stack

3. **Compatibility**
   - 90%+ Graphite CLI command compatibility
   - Seamless migration for Graphite users

## References

- [Graphite CLI Documentation](https://graphite.com/docs/command-reference)
- [Stacked Diffs Guide](https://graphite.com/guides/stacked-diffs)
- [git2 Rust Library](https://docs.rs/git2)
- [GitHub GraphQL API](https://docs.github.com/en/graphql)

Sources:
- [Graphite Command Reference](https://graphite.com/docs/command-reference)
- [How Stacked Diffs Work](https://graphite.com/guides/how-do-stacked-diffs-work)
- [Stacked Diffs on GitHub](https://graphite.com/guides/stacked-diffs-on-github)
