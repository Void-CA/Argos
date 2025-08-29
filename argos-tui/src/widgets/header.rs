use ratatui::{widgets::{Borders, Paragraph}};

pub struct Header {
    text: String,
}

impl Header {
    pub fn new(text: &str) -> Self {
        Self { text: text.into() }
    }

    pub fn render(&self) -> Paragraph {
        Paragraph::new(self.text.clone())
            .block(ratatui::widgets::Block::default().borders(Borders::ALL))
    }
}
