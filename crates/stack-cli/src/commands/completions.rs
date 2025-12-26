//! Shell completions command

use anyhow::Result;
use clap::Args;
use clap_complete::{generate, Shell};
use std::io;

use crate::output;

/// Generate shell completions
#[derive(Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

pub fn execute(args: CompletionsArgs, cmd: &mut clap::Command) -> Result<()> {
    output::info(&format!("Generating completions for {:?}...", args.shell));

    generate(args.shell, cmd, "gt", &mut io::stdout());

    Ok(())
}
