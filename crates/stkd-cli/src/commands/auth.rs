//! Auth command - authenticate with GitHub or GitLab

use anyhow::Result;
use clap::Args;
use stkd_github::{auth as github_auth, DeviceFlow, GitHubClient};
use stkd_gitlab::auth as gitlab_auth;

use crate::output;

#[derive(Args)]
pub struct AuthArgs {
    /// Authenticate with GitHub
    #[arg(long, conflicts_with = "gitlab")]
    github: bool,

    /// Authenticate with GitLab
    #[arg(long, conflicts_with = "github")]
    gitlab: bool,

    /// GitLab host (for self-hosted instances)
    #[arg(long, default_value = "gitlab.com")]
    host: String,

    /// Personal access token
    #[arg(long)]
    token: Option<String>,

    /// OAuth App Client ID (for GitHub device flow)
    #[arg(long)]
    client_id: Option<String>,

    /// Clear stored credentials
    #[arg(long)]
    logout: bool,

    /// Show current auth status
    #[arg(long)]
    status: bool,

    /// Use web-based OAuth flow (GitHub only, requires --client-id)
    #[arg(long)]
    web: bool,
}

pub async fn execute(args: AuthArgs) -> Result<()> {
    // Determine which provider to use
    let provider = if args.github {
        stkd_engine::ProviderType::GitHub
    } else if args.gitlab {
        stkd_engine::ProviderType::GitLab
    } else {
        // Try to auto-detect from current repo
        match stkd_core::Repository::open(".") {
            Ok(repo) => stkd_engine::detect_provider_type(&repo)
                .unwrap_or(stkd_engine::ProviderType::GitHub),
            Err(_) => stkd_engine::ProviderType::GitHub, // Default to GitHub if not in a repo
        }
    };

    match provider {
        stkd_engine::ProviderType::GitHub => execute_github(args).await,
        stkd_engine::ProviderType::GitLab => execute_gitlab(args).await,
    }
}

async fn execute_github(args: AuthArgs) -> Result<()> {
    // Handle logout
    if args.logout {
        github_auth::clear_credentials()?;
        output::success("Logged out from GitHub");
        return Ok(());
    }

    // Handle status check
    if args.status {
        return show_github_status().await;
    }

    // Determine authentication method
    if args.web {
        // OAuth device flow
        let client_id = args.client_id.ok_or_else(|| {
            anyhow::anyhow!(
                "OAuth device flow requires --client-id.\n\
                 Create an OAuth App at https://github.com/settings/developers\n\
                 and provide the Client ID."
            )
        })?;

        return github_oauth_flow(&client_id).await;
    }

    if let Some(token) = args.token {
        // Direct token authentication
        return github_token_auth(token).await;
    }

    // Interactive authentication
    output::info("GitHub Authentication");
    output::info("");
    output::info("Choose authentication method:");
    output::info("  1. Personal Access Token (PAT)");
    output::info("  2. OAuth Device Flow (requires OAuth App)");
    output::info("");

    let choice = output::select(
        "Authentication method",
        &["Personal Access Token", "OAuth Device Flow"],
    );

    match choice {
        Some(0) => {
            // PAT flow
            output::info("");
            output::info("Create a token at: https://github.com/settings/tokens/new");
            output::info("Required scopes: repo, read:org");
            output::info("");

            let token = output::input("GitHub personal access token")
                .ok_or_else(|| anyhow::anyhow!("No token provided"))?;

            github_token_auth(token).await
        }
        Some(1) => {
            // OAuth flow
            output::info("");
            output::info("OAuth device flow requires an OAuth App Client ID.");
            output::info("Create one at: https://github.com/settings/developers");
            output::info("");

            let client_id = output::input("OAuth App Client ID")
                .ok_or_else(|| anyhow::anyhow!("No client ID provided"))?;

            github_oauth_flow(&client_id).await
        }
        _ => {
            output::info("Cancelled");
            Ok(())
        }
    }
}

async fn show_github_status() -> Result<()> {
    if let Some(token) = github_auth::load_credentials()? {
        output::info(&format!(
            "GitHub: Authenticated with {} token",
            match token.token_type {
                github_auth::TokenType::Pat => "personal access",
                github_auth::TokenType::OAuth => "OAuth",
            }
        ));

        // Validate token
        let client = GitHubClient::new(token.to_auth())?;
        if client.validate_token().await? {
            let user = client.current_user().await?;
            output::success(&format!("  Logged in as: {}", user.login));

            if let Some(name) = user.name {
                output::info(&format!("  Name: {}", name));
            }
            if let Some(email) = user.email {
                output::info(&format!("  Email: {}", email));
            }

            // Show token info
            output::info(&format!(
                "  Created: {}",
                token.created_at.format("%Y-%m-%d %H:%M")
            ));
            if let Some(expires) = token.expires_at {
                output::info(&format!("  Expires: {}", expires.format("%Y-%m-%d %H:%M")));
            }
        } else {
            output::warn("  Token is invalid or expired");
            output::hint("  Run 'gt auth --logout --github' and re-authenticate");
        }
    } else {
        output::info("GitHub: Not authenticated");
        output::hint("Run 'gt auth --github' to authenticate with GitHub");
    }

    Ok(())
}

async fn github_token_auth(token: String) -> Result<()> {
    output::info("Validating GitHub token...");

    // Validate token
    let auth_token = github_auth::AuthToken::pat(token);
    let client = GitHubClient::new(auth_token.to_auth())?;

    if !client.validate_token().await? {
        output::error("Invalid token");
        anyhow::bail!("Token validation failed");
    }

    let user = client.current_user().await?;

    // Save credentials
    github_auth::save_credentials(&auth_token)?;

    output::success(&format!("Authenticated with GitHub as {}", user.login));

    Ok(())
}

async fn github_oauth_flow(client_id: &str) -> Result<()> {
    output::info("Starting GitHub OAuth device flow...");

    let flow = DeviceFlow::new(client_id)?;

    // Request repo and read:org scopes for full functionality
    let scope = "repo read:org";

    let token_response = flow.authenticate(scope).await?;

    // Create auth token
    let auth_token = github_auth::AuthToken::oauth(
        token_response.access_token,
        None, // Device flow doesn't provide refresh token
        None, // Device flow tokens don't expire (unless revoked)
    );

    // Validate and get user info
    let client = GitHubClient::new(auth_token.to_auth())?;
    let user = client.current_user().await?;

    // Save credentials
    github_auth::save_credentials(&auth_token)?;

    output::success(&format!("Authenticated with GitHub as {}", user.login));

    Ok(())
}

async fn execute_gitlab(args: AuthArgs) -> Result<()> {
    let host = &args.host;

    // Handle logout
    if args.logout {
        gitlab_auth::remove_credentials(host)?;
        output::success(&format!("Logged out from GitLab ({})", host));
        return Ok(());
    }

    // Handle status check
    if args.status {
        return show_gitlab_status(host).await;
    }

    // OAuth device flow not supported for GitLab in this version
    if args.web {
        output::warn("OAuth device flow is not yet supported for GitLab.");
        output::info("Please use a Personal Access Token instead.");
    }

    if let Some(token) = args.token {
        return gitlab_token_auth(token, host).await;
    }

    // Interactive authentication
    output::info(&format!("GitLab Authentication ({})", host));
    output::info("");
    output::info(&format!(
        "Create a token at: https://{}/- /user_settings/personal_access_tokens",
        host
    ));
    output::info("Required scopes: api, read_user");
    output::info("");

    let token = output::input("GitLab personal access token")
        .ok_or_else(|| anyhow::anyhow!("No token provided"))?;

    gitlab_token_auth(token, host).await
}

async fn show_gitlab_status(host: &str) -> Result<()> {
    if let Some(token) = gitlab_auth::load_credentials(host)? {
        output::info(&format!(
            "GitLab ({}): Authenticated with {} token",
            host, token.token_type
        ));

        // Validate token
        let provider = if host == "gitlab.com" {
            stkd_gitlab::GitLabProvider::new(&token.token)?
        } else {
            stkd_gitlab::GitLabProvider::with_host(&token.token, host)?
        };

        use stkd_provider_api::UserProvider;
        match provider.current_user().await {
            Ok(user) => {
                output::success(&format!("  Logged in as: {}", user.username));
                if let Some(name) = user.name {
                    output::info(&format!("  Name: {}", name));
                }
                if let Some(email) = user.email {
                    output::info(&format!("  Email: {}", email));
                }
            }
            Err(_) => {
                output::warn("  Token is invalid or expired");
                output::hint(&format!(
                    "  Run 'gt auth --logout --gitlab --host {}' and re-authenticate",
                    host
                ));
            }
        }
    } else {
        output::info(&format!("GitLab ({}): Not authenticated", host));
        if host == "gitlab.com" {
            output::hint("Run 'gt auth --gitlab' to authenticate with GitLab");
        } else {
            output::hint(&format!(
                "Run 'gt auth --gitlab --host {}' to authenticate",
                host
            ));
        }
    }

    Ok(())
}

async fn gitlab_token_auth(token: String, host: &str) -> Result<()> {
    output::info(&format!("Validating GitLab token for {}...", host));

    // Validate token by trying to get current user
    let provider = if host == "gitlab.com" {
        stkd_gitlab::GitLabProvider::new(&token)?
    } else {
        stkd_gitlab::GitLabProvider::with_host(&token, host)?
    };

    use stkd_provider_api::UserProvider;
    let user = provider
        .current_user()
        .await
        .map_err(|_| anyhow::anyhow!("Token validation failed"))?;

    // Save credentials
    let auth_token = gitlab_auth::AuthToken::pat(&token, host);
    gitlab_auth::save_credentials(&auth_token)?;

    output::success(&format!(
        "Authenticated with GitLab ({}) as {}",
        host, user.username
    ));

    Ok(())
}
