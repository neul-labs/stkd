//! Stack CLI - Stacked diffs for Git
//!
//! A Graphite-compatible CLI for managing stacked pull requests.

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use tracing_subscriber::EnvFilter;

mod commands;
mod output;

use commands::*;

/// Stack - Stacked diffs for Git
#[derive(Parser)]
#[command(name = "gt")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Enable debug output
    #[arg(long, global = true)]
    debug: bool,

    /// Suppress non-essential output
    #[arg(long, short, global = true)]
    quiet: bool,

    /// Output results as JSON
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // ========================================================================
    // Initialization
    // ========================================================================
    /// Initialize Stack in this repository
    Init(init::InitArgs),

    // ========================================================================
    // Branch Management
    // ========================================================================
    /// Create a new branch on top of current
    Create(create::CreateArgs),

    /// Rename the current branch
    Rename(rename::RenameArgs),

    /// Delete a branch
    Delete(delete::DeleteArgs),

    /// Start tracking an existing branch
    Track(track::TrackArgs),

    /// Stop tracking a branch
    Untrack(untrack::UntrackArgs),

    // ========================================================================
    // Navigation
    // ========================================================================
    /// Move up the stack (toward tip)
    Up(nav::UpArgs),

    /// Move down the stack (toward root)
    Down(nav::DownArgs),

    /// Jump to the stack tip
    Top(nav::TopArgs),

    /// Jump to the stack root
    Bottom(nav::BottomArgs),

    /// Checkout a branch
    Checkout(checkout::CheckoutArgs),

    // ========================================================================
    // Stack Operations
    // ========================================================================
    /// Show the current stack
    Log(log::LogArgs),

    /// Alias for 'log short'
    Ls(log::LogArgs),

    /// Alias for 'log long'
    Ll(log::LogArgs),

    /// Show current branch info
    Info(info::InfoArgs),

    /// Show stack status
    Status(status::StatusArgs),

    // ========================================================================
    // Editing
    // ========================================================================
    /// Amend the current branch
    Modify(modify::ModifyArgs),

    /// Squash commits in the current branch
    Squash(squash::SquashArgs),

    /// Fold staged changes into a previous commit
    Fold(fold::FoldArgs),

    /// Split the current commit into multiple commits
    Split(split::SplitArgs),

    // ========================================================================
    // Synchronization
    // ========================================================================
    /// Sync with remote and restack
    Sync(sync::SyncArgs),

    /// Restack branches onto updated parents
    Restack(restack::RestackArgs),

    /// Submit PRs for the stack
    Submit(submit::SubmitArgs),

    /// Merge the stack (land PRs)
    Land(land::LandArgs),

    // ========================================================================
    // Conflict Resolution
    // ========================================================================
    /// Continue after resolving conflicts
    Continue(continue_cmd::ContinueArgs),

    /// Abort the current operation
    Abort(abort::AbortArgs),

    // ========================================================================
    // History
    // ========================================================================
    /// Undo the last Stack operation
    Undo(undo::UndoArgs),

    /// Redo the last undone operation
    Redo(redo::RedoArgs),

    // ========================================================================
    // Configuration
    // ========================================================================
    /// Authenticate with GitHub
    Auth(auth::AuthArgs),

    /// View or edit configuration
    Config(config::ConfigArgs),

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Install Claude Code skill for Stack
    InstallSkill(install_skill::InstallSkillArgs),

    // ========================================================================
    // Interactive TUI
    // ========================================================================
    /// Launch interactive TUI
    Tui(tui::TuiArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging
    let filter = if cli.debug {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();

    // Set output mode
    output::set_mode(if cli.json {
        output::OutputMode::Json
    } else if cli.quiet {
        output::OutputMode::Quiet
    } else {
        output::OutputMode::Human
    });

    // Acquire repo lock for commands that mutate shared state
    let _repo_lock = match &cli.command {
        Commands::Completions { .. } | Commands::InstallSkill(..) | Commands::Init(..) => None,
        _ => match git2::Repository::discover(".") {
            Ok(git) => {
                let stack_dir = git.path().join("stkd");
                match stkd_core::RepoLock::acquire_at(&stack_dir) {
                    Ok(lock) => Some(lock),
                    Err(e) => {
                        if cli.json {
                            let error_json = serde_json::json!({
                                "error": format!("{:#}", e),
                            });
                            eprintln!("{}", serde_json::to_string_pretty(&error_json).unwrap_or_default());
                        } else {
                            output::error(&format!("{:#}", e));
                            if let Some(hint) = e.hint() {
                                output::hint(hint);
                            }
                        }
                        std::process::exit(1);
                    }
                }
            }
            Err(_) => None,
        },
    };

    // Execute command
    let result = match cli.command {
        // Initialization
        Commands::Init(args) => init::execute(args, cli.json).await,

        // Branch Management
        Commands::Create(args) => create::execute(args, cli.json).await,
        Commands::Rename(args) => rename::execute(args).await,
        Commands::Delete(args) => delete::execute(args).await,
        Commands::Track(args) => track::execute(args).await,
        Commands::Untrack(args) => untrack::execute(args).await,

        // Navigation
        Commands::Up(args) => nav::up(args).await,
        Commands::Down(args) => nav::down(args).await,
        Commands::Top(args) => nav::top(args).await,
        Commands::Bottom(args) => nav::bottom(args).await,
        Commands::Checkout(args) => checkout::execute(args).await,

        // Stack Operations
        Commands::Log(args) => log::execute(args, false, cli.json).await,
        Commands::Ls(args) => log::execute(args, true, cli.json).await,
        Commands::Ll(args) => log::execute_long(args).await,
        Commands::Info(args) => info::execute(args, cli.json).await,
        Commands::Status(args) => status::execute(args, cli.json).await,

        // Editing
        Commands::Modify(args) => modify::execute(args).await,
        Commands::Squash(args) => squash::execute(args).await,
        Commands::Fold(args) => fold::execute(args).await,
        Commands::Split(args) => split::execute(args).await,

        // Synchronization
        Commands::Sync(args) => sync::execute(args, cli.json).await,
        Commands::Restack(args) => restack::execute(args, cli.json).await,
        Commands::Submit(args) => submit::execute(args, cli.json).await,
        Commands::Land(args) => land::execute(args, cli.json).await,

        // Conflict Resolution
        Commands::Continue(args) => continue_cmd::execute(args).await,
        Commands::Abort(args) => abort::execute(args).await,

        // History
        Commands::Undo(args) => undo::execute(args).await,
        Commands::Redo(args) => redo::execute(args).await,

        // Configuration
        Commands::Auth(args) => auth::execute(args).await,
        Commands::Config(args) => config::execute(args).await,

        // Shell Completions
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            completions::execute(completions::CompletionsArgs { shell }, &mut cmd)
        }

        // Install Skill
        Commands::InstallSkill(args) => install_skill::execute(args).await,

        // TUI
        Commands::Tui(args) => tui::execute(args).await,
    };

    if let Err(e) = result {
        if cli.json {
            let error_json = serde_json::json!({
                "error": format!("{:#}", e),
            });
            eprintln!("{}", serde_json::to_string_pretty(&error_json).unwrap_or_default());
        } else {
            // Display the error
            output::error(&format!("{:#}", e));

            // Check if this is a Stack error with a hint
            if let Some(stack_err) = e.downcast_ref::<stkd_core::Error>() {
                if let Some(hint) = stack_err.hint() {
                    output::hint(hint);
                }
            }
        }

        std::process::exit(1);
    }

    Ok(())
}
