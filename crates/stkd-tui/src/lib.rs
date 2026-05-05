//! Stack TUI - Interactive terminal UI for stacked diffs
//!
//! Provides a keyboard-driven interface for browsing stacks,
//! viewing branch/MR status, and executing stack operations.

pub mod app;
pub mod event;
pub mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

/// Run the TUI application.
pub async fn run_tui() -> Result<()> {
    // Initialize app state before touching the terminal so that
    // errors (e.g., NotInitialized) are printed on the regular screen.
    let mut app = App::new()?;

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stderr = io::stderr();
    crossterm::execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Run the event loop
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let mut events = event::EventStream::new();

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        match events.next().await {
            Some(Ok(event::Event::Tick)) => app.on_tick(),
            Some(Ok(event::Event::Crossterm(evt))) => {
                if app.handle_event(evt).await? {
                    break;
                }
            }
            Some(Err(_)) => break,
            None => break,
        }
    }

    Ok(())
}
