//! Auth command

use anyhow::Result;
use clap::Args;
use stack_github::auth;

use crate::output;

#[derive(Args)]
pub struct AuthArgs {
    /// GitHub personal access token
    #[arg(long)]
    token: Option<String>,

    /// Clear stored credentials
    #[arg(long)]
    logout: bool,

    /// Show current auth status
    #[arg(long)]
    status: bool,
}

pub async fn execute(args: AuthArgs) -> Result<()> {
    if args.logout {
        auth::clear_credentials()?;
        output::success("Logged out");
        return Ok(());
    }

    if args.status {
        if let Some(token) = auth::load_credentials()? {
            output::info(&format!("Authenticated with {} token", match token.token_type {
                auth::TokenType::Pat => "personal access",
                auth::TokenType::OAuth => "OAuth",
            }));

            // Validate token
            let client = stack_github::GitHubClient::new(token.to_auth())?;
            if client.validate_token().await? {
                let user = client.current_user().await?;
                output::info(&format!("Logged in as: {}", user.login));
            } else {
                output::warn("Token is invalid or expired");
            }
        } else {
            output::info("Not authenticated");
            output::hint("Run 'gt auth --token <token>' to authenticate");
        }
        return Ok(());
    }

    let token = if let Some(t) = args.token {
        t
    } else {
        output::input("GitHub personal access token")
            .ok_or_else(|| anyhow::anyhow!("No token provided"))?
    };

    // Validate token
    let auth_token = auth::AuthToken::pat(token);
    let client = stack_github::GitHubClient::new(auth_token.to_auth())?;

    if !client.validate_token().await? {
        anyhow::bail!("Invalid token");
    }

    let user = client.current_user().await?;

    // Save credentials
    auth::save_credentials(&auth_token)?;

    output::success(&format!("Authenticated as {}", user.login));

    Ok(())
}
