# MCP Server for AI Agents

The Model Context Protocol (MCP) lets AI agents like Claude Code interact with Stack directly. Instead of describing Git operations in natural language, agents can invoke Stack commands as tools.

---

## What Is MCP?

MCP is a protocol for connecting AI assistants to tools and data sources. It allows Claude Code, Claude Desktop, and other MCP-compatible clients to:

- Read your stack state
- Create and submit branches
- Sync and restack
- Land PRs

This means you can say "create a new branch for the payment API and submit it" and Claude Code will invoke the exact Stack commands needed.

---

## Installing the MCP Server

### Prerequisites

- Stack CLI installed (`gt` in your PATH)
- An MCP-compatible client (Claude Code, Claude Desktop, or custom)

### Claude Code Integration

Stack provides a built-in skill for Claude Code:

```bash
# Install the Stack skill
gt install-skill

# This adds to your Claude Code config:
# ~/.claude/settings.json or ~/.config/claude/settings.json
```

The skill adds these tools to Claude Code:

| Tool | Description |
|------|-------------|
| `gt_init` | Initialize Stack in a repository |
| `gt_create` | Create a new branch |
| `gt_checkout` | Switch to a branch |
| `gt_submit` | Submit branches as PRs |
| `gt_sync` | Sync with remote |
| `gt_restack` | Restack branches |
| `gt_land` | Land approved PRs |
| `gt_log` | Show stack status |
| `gt_modify` | Amend current branch |
| `gt_delete` | Delete a branch |
| `gt_track` | Track an existing branch |
| `gt_config` | Read/write Stack configuration |

### Manual MCP Server Setup

For other MCP clients, configure the server directly:

```json
{
  "mcpServers": {
    "stkd": {
      "command": "stkd-mcp-server",
      "args": [],
      "env": {
        "STKD_REPO_PATH": "/path/to/repo"
      }
    }
  }
}
```

The `stkd-mcp-server` binary ships with the Stack installation.

---

## Available Tools

### `gt_init`

Initialize Stack in the current repository.

```json
{
  "name": "gt_init",
  "arguments": {
    "provider": "github",
    "trunk": "main"
  }
}
```

**Returns:**
```json
{
  "success": true,
  "message": "Initialized Stack with trunk 'main' and provider 'github'"
}
```

---

### `gt_create`

Create a new branch on top of the current branch.

```json
{
  "name": "gt_create",
  "arguments": {
    "branch_name": "feature/new-api",
    "template": "api"
  }
}
```

**Returns:**
```json
{
  "success": true,
  "branch": "feature/new-api",
  "parent": "main",
  "message": "Created and checked out feature/new-api"
}
```

---

### `gt_checkout`

Switch to a tracked branch.

```json
{
  "name": "gt_checkout",
  "arguments": {
    "branch_name": "feature/auth-models"
  }
}
```

**Returns:**
```json
{
  "success": true,
  "branch": "feature/auth-models",
  "stack": ["feature/auth-models", "feature/auth-api", "feature/auth-ui"]
}
```

---

### `gt_submit`

Submit branches as PRs.

```json
{
  "name": "gt_submit",
  "arguments": {
    "from": "feature/auth-models",
    "draft": false,
    "reviewers": ["alice", "bob"],
    "labels": ["enhancement"]
  }
}
```

**Returns:**
```json
{
  "success": true,
  "submitted": [
    {"branch": "feature/auth-models", "pr": 42, "url": "https://github.com/org/repo/pull/42"},
    {"branch": "feature/auth-api", "pr": 43, "url": "https://github.com/org/repo/pull/43"}
  ]
}
```

---

### `gt_sync`

Sync with remote, restack, and clean up merged branches.

```json
{
  "name": "gt_sync",
  "arguments": {
    "prune": true
  }
}
```

**Returns:**
```json
{
  "success": true,
  "fetched": true,
  "restacked": ["feature/auth-api", "feature/auth-ui"],
  "deleted": ["feature/auth-models"],
  "message": "Synced 2 branches, deleted 1 merged branch"
}
```

---

### `gt_restack`

Restack branches onto their updated parents.

```json
{
  "name": "gt_restack",
  "arguments": {
    "current_only": false,
    "dry_run": false
  }
}
```

**Returns:**
```json
{
  "success": true,
  "restacked": ["feature/auth-api", "feature/auth-ui"],
  "conflicts": false
}
```

---

### `gt_land`

Land an approved PR.

```json
{
  "name": "gt_land",
  "arguments": {
    "branch_name": "feature/auth-models",
    "merge_method": "squash"
  }
}
```

**Returns:**
```json
{
  "success": true,
  "pr": 42,
  "merge_commit": "abc123def456",
  "deleted_branch": "feature/auth-models"
}
```

---

### `gt_log`

Show the current stack tree.

```json
{
  "name": "gt_log",
  "arguments": {
    "all": false
  }
}
```

**Returns:**
```json
{
  "success": true,
  "trunk": "main",
  "stacks": [
    {
      "root": "feature/auth-models",
      "branches": [
        {"name": "feature/auth-models", "parent": "main", "pr": 42, "status": "open"},
        {"name": "feature/auth-api", "parent": "feature/auth-models", "pr": 43, "status": "open"},
        {"name": "feature/auth-ui", "parent": "feature/auth-api", "pr": 44, "status": "draft"}
      ]
    }
  ]
}
```

---

### `gt_modify`

Amend the current branch with staged changes.

```json
{
  "name": "gt_modify",
  "arguments": {
    "message": "Update auth models with timestamps"
  }
}
```

**Returns:**
```json
{
  "success": true,
  "branch": "feature/auth-models",
  "restacked_children": ["feature/auth-api", "feature/auth-ui"]
}
```

---

### `gt_delete`

Delete a branch and its Stack tracking.

```json
{
  "name": "gt_delete",
  "arguments": {
    "branch_name": "feature/old-branch",
    "force": false
  }
}
```

**Returns:**
```json
{
  "success": true,
  "deleted": "feature/old-branch"
}
```

---

### `gt_track`

Track an existing Git branch with Stack.

```json
{
  "name": "gt_track",
  "arguments": {
    "branch_name": "feature/existing",
    "parent": "main"
  }
}
```

**Returns:**
```json
{
  "success": true,
  "branch": "feature/existing",
  "parent": "main"
}
```

---

### `gt_config`

Read or write Stack configuration.

```json
{
  "name": "gt_config",
  "arguments": {
    "action": "get",
    "key": "submit.default-reviewers"
  }
}
```

**Returns:**
```json
{
  "success": true,
  "key": "submit.default-reviewers",
  "value": "alice,bob"
}
```

---

## Using with Claude Code

### Example Conversations

**Creating a stack:**

```
User: Create a new stack for adding webhook support with models, API, and tests.

Claude: I'll create a three-branch stack for webhook support.
[Tool: gt_create branch_name=feature/webhook-models]
[Tool: gt_create branch_name=feature/webhook-api]
[Tool: gt_create branch_name=feature/webhook-tests]

Created stack:
main
 └── feature/webhook-models
      └── feature/webhook-api
           └── feature/webhook-tests
```

**Checking status:**

```
User: What's the status of my stacks?

Claude: Let me check your current stacks.
[Tool: gt_log all=true]

You have 2 active stacks:

1. Webhook stack:
   - feature/webhook-models #48 [open, 1 approval]
   - feature/webhook-api    #49 [open]
   - feature/webhook-tests  #50 [draft]

2. Payment stack:
   - feature/payment-delivery #51 [open, 2 approvals, ready to land]
```

**Landing PRs:**

```
User: Land the payment delivery PR.

Claude: Landing feature/payment-delivery.
[Tool: gt_land branch_name=feature/payment-delivery]

Successfully landed PR #51. The branch has been merged and deleted.
```

**Responding to review feedback:**

```
User: Bob wants me to add a retry_count field to the webhook model.

Claude: I'll check out the webhook models branch, add the field, and update the PR.
[Tool: gt_checkout branch_name=feature/webhook-models]
[Tool: gt_modify message="Add retry_count to webhook model"]
[Tool: gt_submit only=feature/webhook-models]

Updated PR #48 with the retry_count field.
```

---

## Using with Other MCP Clients

### Claude Desktop

Add to Claude Desktop configuration:

```json
{
  "mcpServers": {
    "stkd": {
      "command": "stkd-mcp-server"
    }
  }
}
```

Restart Claude Desktop. Stack tools will appear in the tool palette.

### Custom Clients

Any client implementing the MCP protocol can use Stack tools. The server communicates over stdio using JSON-RPC:

```json
// Request
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "gt_log",
    "arguments": {"all": false}
  }
}

// Response
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\n  \"trunk\": \"main\",\n  \"stacks\": [...]\n}"
      }
    ]
  }
}
```

---

## Security Considerations

### Repository Access

The MCP server runs with the same permissions as the user who started it. It can:
- Read any file in the repository
- Execute Git commands
- Push to remotes
- Create and merge PRs

### Scope Limitations

The server only operates within the configured repository path (`STKD_REPO_PATH`). It cannot access files outside this directory.

### Authentication

The MCP server uses the same authentication as the CLI:
- OAuth tokens stored in `~/.config/stkd/credentials/`
- SSH keys for Git operations
- No separate authentication layer

---

## Troubleshooting

### "MCP server not found"

Ensure `stkd-mcp-server` is in your PATH:

```bash
which stkd-mcp-server
# If not found:
cargo install stkd-cli  # Includes MCP server
```

### "Repository not initialized"

The MCP server requires Stack to be initialized:

```bash
cd /your/repo
gt init
```

### "Authentication failed"

Re-authenticate via the CLI:

```bash
gt auth login github
```

The MCP server uses the same credentials.

---

## Tips for AI Agent Workflows

1. **Be specific**: "Submit my current stack" is clearer than "push my changes"
2. **Verify before landing**: Always check PR status before landing — agents can't see CI results directly
3. **Use dry-run for safety**: "Show me what restacking would do" before executing
4. **Sync first**: Tell the agent to sync before creating new branches
5. **Handle conflicts**: If restacking produces conflicts, the agent will report them but you'll need to resolve manually
