use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::data::{capture::Capture, game::Game};

fn capture_from(capture: &Capture) -> Paragraph {
    Paragraph::new(Span::styled(
        format!(" {} ", capture.count().to_string()),
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(capture.ai_side.color())
            .fg(Color::Black),
    ))
    .style(Style::default().fg(capture.ai_side.color()))
    .alignment(Alignment::Center)
}

impl Game {
    pub fn draw_captures<B: Backend>(&self, f: &mut tui::Frame<B>, rect: Rect) {
        let marged = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(30)
            .split(rect)[0];

        let block = Block::default()
            .title(" Captures ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        f.render_widget(block, marged);

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
            .split(marged);

        let for_ai_block = Block::default()
            .title("For AI Captures")
            .title_alignment(Alignment::Center);
        let center_capture_block = Block::default()
            .title("Center")
            .title_alignment(Alignment::Center);
        let against_ai_block = Block::default()
            .title("Against AI Captures")
            .title_alignment(Alignment::Center);

        f.render_widget(
            capture_from(&self.for_ai.as_ref().unwrap().capture).block(for_ai_block),
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
            capture_from(&self.against_ai.as_ref().unwrap().capture).block(against_ai_block),
            capture_chunk[2],
        );
    }
}
