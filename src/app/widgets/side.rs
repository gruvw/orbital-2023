use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::data::game::{AiSide, Game};

impl Game {
    pub fn draw_side<B: Backend>(&self, f: &mut tui::Frame<B>, rect: Rect, ai_side: AiSide) {
        let title_block = Block::default()
            .title(format!(" {} ", ai_side.name()))
            .title_alignment(match ai_side {
                AiSide::For => Alignment::Left,
                AiSide::Against => Alignment::Right,
            })
            .borders(Borders::ALL);

        let paragraph = Paragraph::new(ai_side.name())
            .style(Style::default().fg(ai_side.color()))
            .block(title_block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, rect);
    }
}
