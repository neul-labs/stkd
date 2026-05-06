use crossterm::event::{Event as CrosstermEvent, EventStream as CrosstermEventStream};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time;

/// Application events.
pub enum Event {
    /// Terminal tick event (for animations/updates).
    Tick,
    /// Crossterm event (keyboard, mouse, resize).
    Crossterm(CrosstermEvent),
}

/// Event stream that yields both tick events and crossterm events.
pub struct EventStream {
    crossterm: CrosstermEventStream,
    tick: time::Interval,
}

impl Default for EventStream {
    fn default() -> Self {
        Self::new()
    }
}

impl EventStream {
    pub fn new() -> Self {
        Self {
            crossterm: CrosstermEventStream::new(),
            tick: time::interval(Duration::from_millis(250)),
        }
    }

    pub async fn next(&mut self) -> Option<Result<Event, std::io::Error>> {
        tokio::select! {
            _ = self.tick.tick() => Some(Ok(Event::Tick)),
            evt = self.crossterm.next() => {
                match evt {
                    Some(Ok(evt)) => Some(Ok(Event::Crossterm(evt))),
                    Some(Err(e)) => Some(Err(e)),
                    None => None,
                }
            }
        }
    }
}
