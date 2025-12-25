//! Config command

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct ConfigArgs {
    /// Config key to get/set
    key: Option<String>,

    /// Value to set
    value: Option<String>,
}

pub async fn execute(args: ConfigArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    let config = repo.config();

    if args.key.is_none() {
        // Show all config
        println!("trunk: {}", config.trunk);
        println!("remote: {}", config.remote);

        if let Some(ref gh) = config.github {
            println!("github.owner: {}", gh.owner);
            println!("github.repo: {}", gh.repo);
        }

        println!("submit.draft: {}", config.submit.draft);
        println!("submit.auto_title: {}", config.submit.auto_title);
        println!("sync.delete_merged: {}", config.sync.delete_merged);

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
            "submit.draft" => config.submit.draft.to_string(),
            _ => {
                anyhow::bail!("Unknown config key: {}", key);
            }
        };
        println!("{}", value);
    }

    Ok(())
}
