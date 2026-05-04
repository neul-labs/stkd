//! Install Claude Code skill for Stack

use anyhow::Result;
use clap::Args;

use crate::output;

#[derive(Args)]
pub struct InstallSkillArgs {
    /// Install MCP server configuration as well
    #[arg(long)]
    mcp: bool,
}

const SKILL_CONTENT: &str = r#"---
name: stkd
description: Manage stacked pull requests with the gt CLI. Use this skill whenever the user needs to create, submit, sync, or land stacked branches, manage stacked diffs/PRs, or work with dependent branches on GitHub/GitLab. Also use for branch navigation (up/down/top/bottom), rebasing stacks, or checking stack status.
---

# Stack (stkd) Skill

Stack is a Graphite-compatible CLI for managing stacked pull requests.

## Commands

Use `gt <command>` to invoke operations. When running from an agent context, prefer using `--json` for structured output.

### Initialization
- `gt init [--trunk <branch>] [--yes]` — Initialize Stack in the current repo

### Branch Management
- `gt create <name> [--from-trunk]` — Create a new branch on top of current
- `gt track <branch>` — Track an existing branch
- `gt untrack <branch>` — Stop tracking a branch
- `gt rename <name>` — Rename current branch
- `gt delete <branch>` — Delete a branch

### Navigation
- `gt up [--steps N]` — Move up the stack
- `gt down [--steps N]` — Move down the stack
- `gt top` — Jump to tip
- `gt bottom` — Jump to root
- `gt checkout <branch>` — Checkout a branch

### Stack Operations
- `gt log [--all]` — Show the current stack (use `--json` for parsing)
- `gt status [--verbose] [--fetch]` — Show branch and MR status
- `gt info` — Show current branch info

### Editing
- `gt modify` — Amend current branch
- `gt squash` — Squash commits
- `gt fold` — Fold staged changes into previous commit
- `gt split` — Split current commit

### Synchronization
- `gt sync [--no-delete] [--no-restack] [--dry-run]` — Sync with remote
- `gt restack [--dry-run]` — Restack branches onto updated parents
- `gt submit [--stack] [--draft] [--dry-run]` — Submit PRs/MRs
- `gt land [--stack] [--method squash] [--yes] [--dry-run]` — Merge and land

### Conflict Resolution
- `gt continue` — Continue after resolving conflicts
- `gt abort` — Abort current operation

### History
- `gt undo` — Undo last operation
- `gt redo` — Redo last undone operation

## JSON Output
Append `--json` to any command for structured machine-readable output.
"#;

pub async fn execute(args: InstallSkillArgs) -> Result<()> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not determine home directory. Set HOME environment variable."))?;
    let skills_dir = std::path::PathBuf::from(home).join(".claude").join("skills").join("stkd");

    std::fs::create_dir_all(&skills_dir)?;

    let skill_path = skills_dir.join("SKILL.md");
    std::fs::write(&skill_path, SKILL_CONTENT)?;

    output::success(&format!("Installed Claude skill to {}", skill_path.display()));

    if args.mcp {
        output::info("MCP server installation not yet implemented. Use the stkd-mcp binary directly.");
    }

    Ok(())
}
