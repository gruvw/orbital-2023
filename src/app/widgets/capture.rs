use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Gauge, Paragraph},
};

use crate::app::data::{
    capture::Capture,
    game::{AiSide, Game},
    side::TOTAL_PROGRESS,
};

fn capture_from(capture: &Capture) -> Paragraph {
    Paragraph::new(Span::styled(
        capture.count().to_string(),
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .style(Style::default().fg(capture.ai_side.color()))
    .alignment(Alignment::Center)
}

impl Game {
    pub fn draw_captures<B: Backend>(&self, f: &mut tui::Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(" Captures ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL);
        f.render_widget(block, rect);

        let capture_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .margin(1)
            .split(rect);

        let for_ai_block = Block::default()
            .title("For AI Captures")
            .title_alignment(Alignment::Left);
        let center_capture_block = Block::default()
            .title("Center")
            .title_alignment(Alignment::Center);
        let against_ai_block = Block::default()
            .title("Against AI Captures")
            .title_alignment(Alignment::Right);

        f.render_widget(
            capture_from(&self.for_ai.capture).block(for_ai_block),
            capture_chunk[0],
        );
        match &self.center_capture {
            Some(capture) => f.render_widget(
                capture_from(&capture).block(center_capture_block),
                capture_chunk[1],
            ),
            None => f.render_widget(center_capture_block, capture_chunk[1]),
        };
        f.render_widget(
            capture_from(&self.against_ai.capture).block(against_ai_block),
            capture_chunk[2],
        );
    }
}
