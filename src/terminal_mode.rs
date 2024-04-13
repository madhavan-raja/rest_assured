use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

use crate::config::Config;

pub struct TerminalMode {
    config: Config,
}

impl TerminalMode {
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

    fn handle_events(&self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn ui(&self, frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new(self.config.file.clone())
                .block(Block::default().title("Greeting").borders(Borders::ALL)),
            frame.size(),
        );
    }
}
