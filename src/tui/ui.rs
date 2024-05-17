use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::Tui;

impl Tui {
    pub fn ui(&self, frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new(self.config.file.clone())
                .block(Block::default().title("Greeting").borders(Borders::ALL)),
            frame.size(),
        );
    }
}
