use crossterm::event;
use event::{Event, KeyCode};
use std::io;

use crate::Tui;

impl Tui {
    pub fn handle_events(&self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
