//! Navigation commands

use anyhow::Result;
use clap::Args;
use stack_core::Repository;

use crate::output;

#[derive(Args)]
pub struct UpArgs {
    /// Number of branches to move up
    #[arg(default_value = "1")]
    steps: usize,
}

#[derive(Args)]
pub struct DownArgs {
    /// Number of branches to move down
    #[arg(default_value = "1")]
    steps: usize,
}

#[derive(Args)]
pub struct TopArgs {}

#[derive(Args)]
pub struct BottomArgs {}

pub async fn up(args: UpArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    repo.ensure_clean()?;

    let target = repo.up(args.steps)?;
    output::success(&format!("Switched to {}", output::branch(&target, true)));

    Ok(())
}

pub async fn down(args: DownArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    repo.ensure_clean()?;

    let target = repo.down(args.steps)?;
    output::success(&format!("Switched to {}", output::branch(&target, true)));

    Ok(())
}

pub async fn top(_args: TopArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    repo.ensure_clean()?;

    let target = repo.top()?;
    output::success(&format!("Switched to {}", output::branch(&target, true)));

    Ok(())
}

pub async fn bottom(_args: BottomArgs) -> Result<()> {
    let repo = Repository::open(".")?;
    repo.ensure_clean()?;

    let target = repo.bottom()?;
    output::success(&format!("Switched to {}", output::branch(&target, true)));

    Ok(())
}
