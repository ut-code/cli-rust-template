use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Mode};

/// Event handler for the TUI
pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    /// Poll for the next event
    pub fn next(&self) -> Result<Option<AppEvent>> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                Event::Key(key) => Ok(Some(AppEvent::Key(key))),
                _ => Ok(None),
            }
        } else {
            Ok(Some(AppEvent::Tick))
        }
    }
}

/// Application events
#[derive(Debug)]
pub enum AppEvent {
    /// Key press event
    Key(KeyEvent),
    /// Tick event (periodic update)
    Tick,
}

/// Handle key events and update app state
pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match app.mode {
        Mode::Normal => handle_normal_mode(app, key),
        Mode::Editing => handle_edit_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),

        // Navigation
        KeyCode::Down | KeyCode::Char('j') => app.list_next(),
        KeyCode::Up | KeyCode::Char('k') => app.list_previous(),

        // Counter
        KeyCode::Right | KeyCode::Char('l') => app.increment_counter(),
        KeyCode::Left | KeyCode::Char('h') => app.decrement_counter(),

        // Actions
        KeyCode::Char('d') | KeyCode::Delete => app.delete_selected(),
        KeyCode::Char('a') => app.add_item(format!("New Item {}", app.items.len() + 1)),
        KeyCode::Enter => app.start_edit(),

        _ => {}
    }
}

fn handle_edit_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => app.confirm_edit(),
        KeyCode::Esc => app.cancel_edit(),
        KeyCode::Backspace => app.edit_pop(),
        KeyCode::Char(c) => app.edit_push(c),
        _ => {}
    }
}
