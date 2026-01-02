//! Config command

use anyhow::Result;
use clap::Args;
use colored::Colorize;
use stack_core::config::CONFIG_VERSION;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct ConfigArgs {
    /// Config key to get/set
    key: Option<String>,

    /// Value to set
    value: Option<String>,

    /// Show raw JSON config
    #[arg(long)]
    json: bool,
}

pub async fn execute(args: ConfigArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let config = repo.config();

    // Show raw JSON if requested
    if args.json {
        let json = serde_json::to_string_pretty(&config)?;
        println!("{}", json);
        return Ok(());
    }

    if args.key.is_none() {
        // Show all config
        println!("{}", "Stack Configuration".bold());
        println!();

        // Version info
        println!("{}", "Version".dimmed());
        println!("  version: {} (current: {})", config.version, CONFIG_VERSION);
        println!();

        // Core settings
        println!("{}", "Core".dimmed());
        println!("  trunk: {}", config.trunk);
        println!("  remote: {}", config.remote);
        println!();

        // Provider config (new v2 format)
        println!("{}", "Provider".dimmed());
        if let Some(ref provider) = config.provider {
            println!("  type: {}", provider.provider_type);
            if let Some(ref owner) = provider.owner {
                println!("  owner: {}", owner);
            }
            if let Some(ref repo_name) = provider.repo {
                println!("  repo: {}", repo_name);
            }
            if let Some(ref host) = provider.host {
                println!("  host: {}", host);
            }
            if let Some(ref api_url) = provider.api_url {
                println!("  api_url: {}", api_url);
            }
        } else if let Some(ref gh) = config.github {
            // Legacy GitHub config (will be migrated on next load)
            println!("  {} (legacy format - will be migrated)", "github".yellow());
            println!("  owner: {}", gh.owner);
            println!("  repo: {}", gh.repo);
        } else {
            println!("  {} (auto-detected from remote)", "auto".dimmed());
        }
        println!();

        // Submit settings
        println!("{}", "Submit".dimmed());
        println!("  draft: {}", config.submit.draft);
        println!("  auto_title: {}", config.submit.auto_title);
        println!("  pr_template: {}", config.submit.pr_template);
        println!("  include_stack_info: {}", config.submit.include_stack_info);
        println!();

        // Sync settings
        println!("{}", "Sync".dimmed());
        println!("  delete_merged: {}", config.sync.delete_merged);
        println!("  prompt_delete: {}", config.sync.prompt_delete);
        println!("  auto_restack: {}", config.sync.auto_restack);
        println!("  pull_trunk: {}", config.sync.pull_trunk);

        output::hint("Edit .git/stack/config.json to modify settings");
        output::hint("Use --json to see the raw config file");

        return Ok(());
    }

    let key = args.key.unwrap();

    if let Some(value) = args.value {
        // Set config
        output::info(&format!("Setting {} = {}", key, value));
        output::warn("Config modification not yet implemented");
        output::hint("Edit .git/stack/config.json directly");
    } else {
        // Get config
        let value = match key.as_str() {
            "trunk" => config.trunk.clone(),
            "remote" => config.remote.clone(),
            "version" => config.version.to_string(),
            "submit.draft" => config.submit.draft.to_string(),
            "submit.auto_title" => config.submit.auto_title.to_string(),
            "sync.delete_merged" => config.sync.delete_merged.to_string(),
            "provider.type" => config.provider
                .as_ref()
                .map(|p| p.provider_type.to_string())
                .unwrap_or_else(|| "auto".to_string()),
            "provider.owner" => config.effective_provider()
                .and_then(|p| p.owner)
                .unwrap_or_default(),
            "provider.repo" => config.effective_provider()
                .and_then(|p| p.repo)
                .unwrap_or_default(),
            _ => {
                anyhow::bail!("Unknown config key: {}", key);
            }
        };
        println!("{}", value);
    }

    Ok(())
}
