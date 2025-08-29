use ratatui::{
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
};

pub struct Footer {
    text: String,
}

impl Footer {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }

    pub fn render(&self) -> Paragraph {
        Paragraph::new(self.text.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Gray))
            )
            .style(Style::default().fg(Color::White))
    }
}