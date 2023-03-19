use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{
    data::game::{AiSide, MAX_PLAYERS, MIN_PLAYERS},
    App, AppState,
};

impl App<'_> {
    pub fn draw_setup<B: Backend>(&mut self, f: &mut tui::Frame<B>, rect: Rect, ai_side: AiSide) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(20)
            .constraints(
                [
                    Constraint::Length(3), // For ai
                    Constraint::Length(3), // Against ai
                    Constraint::Length(3), // VPN positions
                    Constraint::Length(3), // Capture positions
                    Constraint::Min(2),
                ]
                .as_ref(),
            )
            .split(rect);

        let white = Style::default()
            .add_modifier(Modifier::ITALIC)
            .fg(tui::style::Color::White);

        let team_block = Block::default()
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL);

        let for_ai_block = team_block
            .clone()
            .title(format!(" {} ", AiSide::For.name()));
        let against_ai_block = team_block.title(format!(" {} ", AiSide::Against.name()));

        let nb_player_pr = format!("Enter digit between {} and {}", MIN_PLAYERS, MAX_PLAYERS);

        let for_ai_nb = match self.game.for_ai.as_ref() {
            Some(side) => Span::styled(
                side.nb_players.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            None => Span::styled(nb_player_pr.clone(), white),
        };

        let against_ai_nb = match self.game.against_ai.as_ref() {
            Some(side) => Span::styled(
                side.nb_players.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            None => Span::styled(
                match (ai_side, self.game.for_ai.as_ref()) {
                    (AiSide::For, None) => "...".to_string(),
                    (AiSide::For, Some(_)) => "Press Enter for next".to_string(),
                    (AiSide::Against, _) => nb_player_pr,
                },
                white,
            ),
        };

        let for_ai_nb_player_prompt = Paragraph::new(Spans::from(vec![
            Span::from("Number of players: "),
            for_ai_nb,
        ]))
        .block(for_ai_block.style(Style::default().fg(AiSide::For.color())));

        let against_ai_nb_player_prompt = Paragraph::new(Spans::from(vec![
            Span::from("Number of players: "),
            against_ai_nb,
        ]))
        .block(against_ai_block.style(Style::default().fg(AiSide::Against.color())));

        f.render_widget(for_ai_nb_player_prompt, chunks[0]);
        f.render_widget(against_ai_nb_player_prompt, chunks[1]);

        let vpn_pos = match (
            &self.state,
            self.game.against_ai.as_ref(),
            &self.vpn_positions,
        ) {
            (AppState::PlayerInput(_), None, _) => Span::styled("...", white),
            (AppState::PlayerInput(_), Some(_), None) => {
                Span::styled("Press Enter to generate VPN postions", white)
            }
            (_, _, Some((p1, p2))) => Span::styled(
                format!("{} {}", p1.to_string(), p2.to_string()),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            _ => panic!("Should never happen"),
        };

        let vpn_block = Block::default()
            .title(" VPN ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL);

        let vpn_positions =
            Paragraph::new(Spans::from(vec![Span::from("VPN positions: "), vpn_pos]))
                .block(vpn_block);

        f.render_widget(
            vpn_positions.style(Style::default().fg(Color::Magenta)),
            chunks[2],
        );

        let capture_pos = match (&self.state, &self.vpn_positions, &self.capture_positions) {
            (AppState::PlayerInput(_), _, _) => Span::styled("...", white),
            (AppState::VPNPositions, _, _) => {
                Span::styled("Press Enter to generate Capture postions", white)
            }
            (AppState::CapturePositions, _, Some((p1, p2, p3))) => Span::styled(
                format!("{} {} {}", p1.to_string(), p2.to_string(), p3.to_string()),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            _ => panic!("Should never happen"),
        };

        let capture_block = Block::default()
            .title(" Captures ")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL);

        let capture_positions = Paragraph::new(Spans::from(vec![
            Span::from("Capture point positions: "),
            capture_pos,
        ]))
        .block(capture_block);

        f.render_widget(
            capture_positions.style(Style::default().fg(Color::Yellow)),
            chunks[3],
        );
    }
}
