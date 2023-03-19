use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::data::game::Game;

impl Game {
    pub fn draw_keys<B: Backend>(&self, f: &mut tui::Frame<B>, rect: Rect) {
        let keys_block = Block::default()
            .title(" Keys ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        f.render_widget(keys_block, rect);

        let keys_cols = Layout::default()
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

        let key_style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Rgb(138, 138, 138));

        let col1 = vec![
            Spans::from(vec![
                Span::styled("[ESC]", key_style),
                Span::styled(": Exit game", Style::default().add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(vec![
                Span::styled("[TAB]", key_style),
                Span::from(": End turn"),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("[F]", key_style),
                Span::from(": Start a challenge (Get ready!)"),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("[1]", key_style),
                Span::from(": For AI team won race"),
            ]),
            Spans::from(vec![
                Span::styled("[0]", key_style),
                Span::from(": Against AI team won race"),
            ]),
        ];
        let col2 = vec![
            Spans::from(vec![
                Span::styled("[A]", key_style),
                Span::from(": Increase center capture for AI team"),
            ]),
            Spans::from(vec![
                Span::styled("[O]", key_style),
                Span::from(": Decrease center capture for AI team"),
            ]),
            Spans::from(vec![
                Span::styled("[S]", key_style),
                Span::from(": Increase center capture for non-AI team"),
            ]),
            Spans::from(vec![
                Span::styled("[N]", key_style),
                Span::from(": Decrease center capture for non-AI team"),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("[5]", key_style),
                Span::from(": Decrease points for AI team"),
            ]),
            Spans::from(vec![
                Span::styled("[7]", key_style),
                Span::from(": Decrease points for non-AI team"),
            ]),
        ];
        let col3 = vec![
            Spans::from(vec![
                Span::styled("[E]", key_style),
                Span::from(": Increase capture for AI team"),
            ]),
            Spans::from(vec![
                Span::styled("[U]", key_style),
                Span::from(": Decrease capture for AI team"),
            ]),
            Spans::from(vec![
                Span::styled("[T]", key_style),
                Span::from(": Increase capture for non-AI team"),
            ]),
            Spans::from(vec![
                Span::styled("[H]", key_style),
                Span::from(": Decrease capture for non-AI team"),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::styled("[I]", key_style),
                Span::from(": For AI team placed Database"),
            ]),
            Spans::from(vec![
                Span::styled("[D]", key_style),
                Span::from(": Against AI team placed Database"),
            ]),
        ];

        f.render_widget(
            Paragraph::new(col1).alignment(Alignment::Left),
            keys_cols[0],
        );
        f.render_widget(
            Paragraph::new(col2).alignment(Alignment::Left),
            keys_cols[1],
        );
        f.render_widget(
            Paragraph::new(col3).alignment(Alignment::Left),
            keys_cols[2],
        );
    }
}
