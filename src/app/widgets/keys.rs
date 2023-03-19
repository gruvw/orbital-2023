use std::fmt::Debug;

use chrono::Duration;
use tui::{
    backend::Backend,
    layout::{
        Alignment, Constraint,
        Direction::{self, Horizontal},
        Layout, Rect,
    },
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::data::{
    game::{AiSide, Game},
    race::{self, Race, SECS_IN_MIN},
};

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

        let col1 = vec![
            Spans::from("[ESC]: exit game"),
            Spans::from("[TAB]: end turn"),
            Spans::from(""),
            Spans::from("[F]: start a challenge"), // TODO
            Spans::from(""),
            Spans::from("[1]: For AI team won race"),
            Spans::from("[0]: Against AI team won race"),
        ];
        let col2 = vec![
            Spans::from("[A]: Increase center capture for AI team"),
            Spans::from("[O]: Decrease center capture for AI team"),
            Spans::from("[S]: Increase center capture for non-AI team"),
            Spans::from("[C]: Decrease center capture for non-AI team"),
            Spans::from(""),
            Spans::from("[5] Decrease points for AI team"),
            Spans::from("[7] Decrease points for non-AI team"),
        ];
        let col3 = vec![
            Spans::from("[E]: Increase capture for AI team"),
            Spans::from("[U]: Decrease capture for AI team"),
            Spans::from("[T]: Increase capture for non-AI team"),
            Spans::from("[H]: Decrease capture for non-AI team"),
            Spans::from(""),
            Spans::from("[I]: For AI team placed Database"),
            Spans::from("[D]: Against AI team placed Database"),
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
