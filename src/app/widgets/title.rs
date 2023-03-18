use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Drawable};

pub struct Title<'a> {
    text: &'a str,
}

impl Title<'_> {
    pub fn new(title: &str) -> Title {
        Title { text: title }
    }
}

impl Drawable for Title<'_> {
    fn draw<B: Backend>(&self, _app: &App, f: &mut tui::Frame<B>, rect: Rect) {
        let title_block = Block::default()
            .title_alignment(Alignment::Center)
            .borders(Borders::BOTTOM);
        let paragraph = Paragraph::new(self.text)
            .block(title_block)
            .alignment(Alignment::Center);
        f.render_widget(paragraph, rect);
    }
}
