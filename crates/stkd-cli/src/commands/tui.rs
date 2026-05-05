//! Launch the interactive TUI

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct TuiArgs {}

pub async fn execute(_args: TuiArgs) -> Result<()> {
    stkd_tui::run_tui().await
}
