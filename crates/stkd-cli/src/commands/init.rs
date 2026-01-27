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

pub async fn execute(args: InitArgs) -> Result<()> {
    // Check if already initialized
    if Repository::open(".").is_ok() {
        output::warn("Stack is already initialized in this repository");
        output::hint("Use 'gt config' to view or modify settings");
        return Ok(());
    }

    // Open the git repository to detect settings
    let git_repo = git2::Repository::open(".")
        .context("Not a git repository. Run 'git init' first.")?;

    println!("{}", "Stack Configuration Wizard".bold());
    println!();

    // Detect and confirm trunk branch
    let detected_trunk = detect_trunk(&git_repo);
    let trunk = if args.yes {
        args.trunk.unwrap_or(detected_trunk)
    } else if let Some(t) = args.trunk {
        t
    } else {
        println!("{}", "Trunk Branch".dimmed());
        println!("  Detected: {}", detected_trunk.green());

        if output::confirm(&format!("Use '{}' as trunk branch?", detected_trunk)) {
            detected_trunk
        } else {
            output::input("Enter trunk branch name")
                .unwrap_or_else(|| detected_trunk.clone())
        }
    };

    // Detect and confirm remote
    let detected_remote = args.remote.clone().unwrap_or_else(|| detect_remote(&git_repo));
    let remote = if args.yes {
        detected_remote
    } else {
        println!();
        println!("{}", "Remote".dimmed());
        println!("  Detected: {}", detected_remote.green());

        if output::confirm(&format!("Use '{}' as remote?", detected_remote)) {
            detected_remote
        } else {
            output::input("Enter remote name")
                .unwrap_or_else(|| detected_remote.clone())
        }
    };

    // Detect provider from remote URL
    let provider_config = if let Ok(remote_obj) = git_repo.find_remote(&remote) {
        if let Some(url) = remote_obj.url() {
            ProviderConfig::from_remote_url(url)
        } else {
            None
        }
    } else {
        None
    };

    // Show detected provider info
    if !args.yes {
        if let Some(ref provider) = provider_config {
            println!();
            println!("{}", "Provider".dimmed());
            println!("  Type: {}", provider.provider_type.to_string().green());
            if let Some(ref owner) = provider.owner {
                println!("  Owner: {}", owner);
            }
            if let Some(ref repo_name) = provider.repo {
                println!("  Repo: {}", repo_name);
            }
            if let Some(ref host) = provider.host {
                println!("  Host: {}", host);
            }
        }
    }

    // Ask about default settings
    let (draft_default, delete_merged) = if args.yes {
        (false, true)
    } else {
        println!();
        println!("{}", "Default Settings".dimmed());

        let draft = output::confirm("Create PRs/MRs as draft by default?");
        let delete = output::confirm("Delete local branches after merge?");

        (draft, delete)
    };

    // Create config
    let mut config = StackConfig::default();
    config.trunk = trunk.clone();
    config.remote = remote.clone();
    config.provider = provider_config;
    config.submit.draft = draft_default;
    config.sync.delete_merged = delete_merged;

    // Initialize Stack with custom config
    let repo = Repository::init_with_config(".", config)?;

    println!();
    output::success("Stack initialized!");
    println!();
    println!("{}", "Configuration".dimmed());
    println!("  Trunk: {}", repo.trunk().green());
    println!("  Remote: {}", remote);
    if let Some(provider) = repo.config().effective_provider() {
        println!("  Provider: {}", provider.provider_type);
    }
    println!();

    // Check authentication status
    check_auth_status(&repo).await;

    output::hint("Run 'gt create <branch>' to start your first stack");

    Ok(())
}

fn detect_trunk(repo: &git2::Repository) -> String {
    // Check common trunk branch names
    for candidate in &["main", "master", "develop", "trunk"] {
        if repo
            .find_branch(candidate, git2::BranchType::Local)
            .is_ok()
        {
            return candidate.to_string();
        }
    }
    "main".to_string()
}

fn detect_remote(repo: &git2::Repository) -> String {
    if let Ok(remotes) = repo.remotes() {
        if remotes.iter().any(|r| r == Some("origin")) {
            return "origin".to_string();
        }
        if let Some(Some(first)) = remotes.iter().next() {
            return first.to_string();
        }
    }
    "origin".to_string()
}

async fn check_auth_status(repo: &Repository) {
    use crate::provider_context::{detect_provider_type, ProviderType};

    if let Ok(provider_type) = detect_provider_type(repo) {
        let authenticated = match provider_type {
            ProviderType::GitHub => {
                stkd_github::auth::load_credentials()
                    .ok()
                    .flatten()
                    .is_some()
            }
            ProviderType::GitLab => {
                let host = repo
                    .config()
                    .effective_provider()
                    .and_then(|p| p.host)
                    .unwrap_or_else(|| "gitlab.com".to_string());
                stkd_gitlab::auth::load_credentials(&host)
                    .ok()
                    .flatten()
                    .is_some()
            }
        };

        if !authenticated {
            output::warn(&format!("Not authenticated with {}", provider_type));
            output::hint(&format!(
                "Run 'gt auth --{}' to authenticate",
                match provider_type {
                    ProviderType::GitHub => "github",
                    ProviderType::GitLab => "gitlab",
                }
            ));
            println!();
        }
    }
}
