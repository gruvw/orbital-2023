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
    game::Game,
    race::{self, SECS_IN_MIN},
};

impl Game {
    pub fn draw_race<B: Backend>(&self, f: &mut tui::Frame<B>, rect: Rect) {
        let marged = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(45)
            .split(rect)[0];

        let mut race_block = Block::default()
            .title(" Race ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL);
        if let Some(_) = &self.race {
            race_block = race_block.style(Style::default().fg(Color::Magenta));
        };
        f.render_widget(race_block, marged);

        let chunks = Layout::default()
            .direction(Horizontal)
            .constraints(if let Some(_) = &self.race {
                [Constraint::Ratio(3, 4), Constraint::Ratio(1, 4)]
            } else {
                [Constraint::Ratio(4, 4), Constraint::Ratio(0, 4)]
            })
            .margin(1)
            .split(marged);

        let text = match &self.race {
            Some(race) => vec![
                Span::from("Rush now to "),
                Span::styled(
                    race.position.to_string(),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Yellow),
                ),
                Span::from(format!(
                    " ! First team there gains {} points.",
                    race::RACE_POINTS,
                )),
            ],
            None => vec![Span::styled("No race for the moment", Style::default())],
        };

        let paragraph = Paragraph::new(Spans::from(text)).alignment(Alignment::Center);

        f.render_widget(paragraph, chunks[0]);

        if let Some(race) = &self.race {
            let remaining = race.remaining_time();
            let text = format!(
                " {}:{} ",
                remaining.num_minutes(),
                remaining.num_seconds() as u64 % SECS_IN_MIN
            );
            let paragraph = Paragraph::new(Span::styled(
                text,
                Style::default().bg(Color::Magenta).fg(Color::Black),
            ))
            .alignment(Alignment::Center);
            f.render_widget(paragraph, chunks[1]);
        }
    }
}
