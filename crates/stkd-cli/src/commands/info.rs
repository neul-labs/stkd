//! Info command

use anyhow::Result;
use clap::Args;
use serde::Serialize;
use stkd_core::Repository;

use crate::output;

#[derive(Args)]
pub struct InfoArgs {
    /// Branch to show info for (defaults to current)
    branch: Option<String>,
}

#[derive(Serialize)]
struct InfoJson {
    branch: String,
    parent: String,
    status: String,
    children: Vec<String>,
    mr_number: Option<u64>,
    mr_url: Option<String>,
    created_at: String,
    updated_at: String,
}

pub async fn execute(args: InfoArgs, json: bool) -> Result<()> {
    let repo = Repository::open(".")?;

    let branch = args
        .branch
        .or_else(|| repo.current_branch().ok().flatten())
        .ok_or_else(|| anyhow::anyhow!("No branch specified and not on a branch"))?;

    let info = repo
        .storage()
        .load_branch(&branch)?
        .ok_or_else(|| anyhow::anyhow!("Branch '{}' is not tracked", branch))?;

    if json {
        let info_json = InfoJson {
            branch: info.name.clone(),
            parent: info.parent.clone(),
            status: format!("{:?}", info.status),
            children: info.children.clone(),
            mr_number: info.merge_request_id,
            mr_url: info.merge_request_url.clone(),
            created_at: info.created_at.format("%Y-%m-%d %H:%M").to_string(),
            updated_at: info.updated_at.format("%Y-%m-%d %H:%M").to_string(),
        };
        println!("{}", serde_json::to_string_pretty(&info_json)?);
    } else {
        println!("Branch: {}", output::branch(&info.name, true));
        println!("Parent: {}", info.parent);
        println!("Status: {}", info.status);

        if !info.children.is_empty() {
            println!("Children: {}", info.children.join(", "));
        }

        if let Some(pr) = info.merge_request_id {
            println!("PR: #{}", pr);
            if let Some(ref url) = info.merge_request_url {
                println!("URL: {}", url);
            }
        }

        println!("Created: {}", info.created_at.format("%Y-%m-%d %H:%M"));
        println!("Updated: {}", info.updated_at.format("%Y-%m-%d %H:%M"));
    }

    Ok(())
}
