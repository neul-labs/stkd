//! Initialize Stack in a repository

use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use stkd_core::config::{ProviderConfig, StackConfig};
use stkd_core::Repository;

use crate::output;

#[derive(Args)]
pub struct InitArgs {
    /// Trunk branch name (auto-detected if not specified)
    #[arg(long)]
    trunk: Option<String>,

    /// Skip interactive wizard
    #[arg(long, short = 'y')]
    yes: bool,

    /// Remote name (default: origin)
    #[arg(long)]
    remote: Option<String>,
}

pub async fn execute(args: InitArgs, json: bool) -> Result<()> {
    // Check if already initialized
    if Repository::open(".").is_ok() {
        if json {
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({"error": "Stack is already initialized in this repository" }))?);
        } else {
            output::warn("Stack is already initialized in this repository");
            output::hint("Use 'gt config' to view or modify settings");
        }
        return Ok(());
    }

    let opts = stkd_engine::InitOptions {
        trunk: args.trunk,
        remote: args.remote,
        draft_default: false,
        delete_merged: true,
    };

    let result = stkd_engine::init(".", opts)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", "Stack Configuration Wizard".bold());
        println!();
        println!("{}", "Configuration".dimmed());
        println!("  Trunk: {}", result.trunk.green());
        println!("  Remote: {}", result.remote);
        if let Some(provider) = result.provider {
            println!("  Provider: {}", provider.provider_type);
        }
        println!();
        output::success("Stack initialized!");
        output::hint("Run 'gt create <branch>' to start your first stack");
    }

    Ok(())
}
