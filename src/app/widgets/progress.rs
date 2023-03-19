use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge},
};

use crate::app::data::{
    game::{AiSide, Game},
    side::TOTAL_PROGRESS,
};

fn gauge_from(game: &Game, ai_side: AiSide) -> Gauge {
    let progress = (match ai_side {
        AiSide::For => game.for_ai.progress,
        AiSide::Against => game.against_ai.progress,
    } as f64)
        / (TOTAL_PROGRESS as f64);
    let label = format!("{:.2}%", progress * 100.0);
    Gauge::default()
        .gauge_style(
            Style::default()
                .fg(ai_side.color())
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .label(label)
        .ratio(progress)
}

impl Game {
    pub fn draw_progress<B: Backend>(&self, f: &mut tui::Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(" Progress ")
            .style(Style::default().fg(Color::Yellow))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);
        f.render_widget(block, rect);

        let progress_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)].as_ref())
            .margin(1)
            .split(rect);

        f.render_widget(gauge_from(self, AiSide::For), progress_chunks[0]);
        f.render_widget(gauge_from(self, AiSide::Against), progress_chunks[1]);
    }
}
