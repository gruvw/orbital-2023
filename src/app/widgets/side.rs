use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
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
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ai_side.color()));
        f.render_widget(title_block, rect);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(4),
                ]
                .as_ref(),
            )
            .margin(2)
            .split(rect);

        let turn = Paragraph::new(ai_side.name())
            .style(if self.turn == ai_side {
                Style::default().bg(ai_side.color()).fg(Color::Black)
            } else {
                Style::default().fg(ai_side.color())
            })
            .alignment(Alignment::Center);

        f.render_widget(turn, chunks[0]);

        let side = match ai_side {
            AiSide::For => self.for_ai.as_ref(),
            AiSide::Against => self.against_ai.as_ref(),
        }
        .unwrap();

        let style = Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(ai_side.color());

        let desc = Paragraph::new(vec![
            Spans::from(vec![
                Span::from("Number of player: "),
                Span::styled(format!("{}", side.nb_players), style),
            ]),
            Spans::from(vec![
                Span::from("Number of rounds: "),
                Span::styled(format!("{}", side.nb_rounds), style),
            ]),
            Spans::from(vec![
                Span::from("Points (progress): "),
                Span::styled(format!("{}", side.progress()), style),
            ]),
        ])
        .alignment(Alignment::Left);

        f.render_widget(desc, chunks[2]);
    }
}
