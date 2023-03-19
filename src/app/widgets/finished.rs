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
    pub fn draw_finished<B: Backend>(
        &mut self,
        f: &mut tui::Frame<B>,
        rect: Rect,
        ai_side: AiSide,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(10), // Art
                    Constraint::Length(1),
                    Constraint::Length(7), // Success msg
                    Constraint::Length(2),
                    Constraint::Length(4 + 5), // Credits
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .horizontal_margin(50)
            .vertical_margin(10)
            .split(rect);

        let art_style = Style::default().fg(Color::Magenta);

        let art_msg = Paragraph::new(vec![
            Spans::from(Span::styled(
                " _____       _               _____                             _   ",
                art_style,
            )),
            Spans::from(Span::styled(
                r"/  __ \     | |             /  __ \                           | |  ",
                art_style,
            )),
            Spans::from(Span::styled(
                r"| /  \/_   _| |__   ___ _ __| /  \/ ___  _ __  _ __   ___  ___| |_ ",
                art_style,
            )),
            Spans::from(Span::styled(
                r"| |   | | | | '_ \ / _ \ '__| |    / _ \| '_ \| '_ \ / _ \/ __| __|",
                art_style,
            )),
            Spans::from(Span::styled(
                r"| \__/\ |_| | |_) |  __/ |  | \__/\ (_) | | | | | | |  __/ (__| |_ ",
                art_style,
            )),
            Spans::from(Span::styled(
                r" \____/\__, |_.__/ \___|_|   \____/\___/|_| |_|_| |_|\___|\___|\__|",
                art_style,
            )),
            Spans::from(Span::styled(
                r"        __/ |                                                      ",
                art_style,
            )),
            Spans::from(Span::styled(
                r"       |___/                                                       ",
                art_style,
            )),
        ]);

        f.render_widget(art_msg.alignment(Alignment::Center), chunks[0]);

        let success_block = Block::default()
            .title(" The END ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let success_msg = Paragraph::new(vec![
            Spans::from(""),
            Spans::from(vec![
                Span::from("Well done! The "),
                Span::styled(
                    ai_side.name(),
                    Style::default()
                        .fg(ai_side.color())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::from(" team won the game!"),
            ]),
            Spans::from(vec![
                Span::from("Thanks for playing, we hope you enjoyed"),
                Span::styled(" :)", Style::default().fg(Color::Green)),
            ]),
            Spans::from("Press [ESC] to quit."),
        ])
        .alignment(Alignment::Center);

        f.render_widget(success_msg.block(success_block), chunks[2]);

        let credits_chucks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(23)].as_ref())
            .horizontal_margin(20)
            .split(chunks[4]);

        let credits_block = Block::default()
            .title("Credits")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let credits_style = Style::default().fg(Color::Yellow);

        let credits_msg = Paragraph::new(vec![
            Spans::from(""),
            Spans::from(Span::styled("Lucas Jung (@gruvw)", credits_style)),
            Spans::from(Span::styled("Florian Kolly", credits_style)),
            Spans::from(Span::styled("Arnaud Haizmann", credits_style)),
            Spans::from(Span::styled("Chloé Chochon", credits_style)),
            Spans::from(Span::styled("Tifaine Mezencev", credits_style)),
        ]);

        f.render_widget(
            credits_msg
                .alignment(Alignment::Center)
                .block(credits_block),
            credits_chucks[0],
        );

        // let credits_block = Block::default()
        //     .title("Credits")
        //     .title_alignment(Alignment::Left)
        //     .borders(Borders::ALL);
    }
}
