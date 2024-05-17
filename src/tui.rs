mod handle_events;
mod ui;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::{self, stdout};

use crate::config::Config;

pub struct Tui {
    config: Config,
}

impl Tui {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn start(&self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let mut should_quit = false;
        while !should_quit {
            terminal.draw(|frame| self.ui(frame))?; // Pass the frame argument to the ui method
            should_quit = self.handle_events()?;
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
