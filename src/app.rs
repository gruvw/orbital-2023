mod data;
mod widgets;

use std::borrow::BorrowMut;

use crossterm::event::KeyCode;

use rand::Rng;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use self::{
    data::{
        capture::capture_position,
        game::{AiSide, Game, Position, MAX_PLAYERS, MIN_PLAYERS},
        side::Side,
        vpn::vpn_position,
    },
    widgets::title::Title,
};

pub trait Drawable {
    fn draw<B: Backend>(&self, app: &App, f: &mut Frame<B>, rect: Rect);
}
enum InputMode {
    Normal,
    Insert,
}

#[derive(Clone)]
pub enum AppState {
    PlayerInput(AiSide),
    VPNPositions,
    CapturePositions,
    Play,
}

// pub struct Setup {
//     nb_player: String,
// }

pub struct App<'a> {
    title: Title<'a>,
    game: Game,
    state: AppState,
    input_mode: InputMode,
    vpn_positions: Option<(Position, Position)>,
    capture_positions: Option<(Position, Position, Position)>,
    pub should_quit: bool,
}

impl App<'_> {
    pub fn new() -> App<'static> {
        App {
            title: Title::new("Welcome to the best Game CyberConnect!"),
            game: Game::new(),
            state: AppState::PlayerInput(AiSide::For),
            input_mode: InputMode::Normal,
            // input: String::new(),
            should_quit: false,
            vpn_positions: None,
            capture_positions: None,
        }
    }
}

impl App<'_> {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(10)].as_ref())
            .split(f.size());

        self.title.draw(self, f, chunks[0]);

        match self.state.clone() {
            AppState::PlayerInput(ai_side) => {
                self.input_mode = InputMode::Insert;
                self.draw_setup(f, chunks[1], ai_side);
                return;
            }
            AppState::VPNPositions | AppState::CapturePositions => {
                self.draw_setup(f, chunks[1], AiSide::For);
            }
            AppState::Play => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(4),
                            Constraint::Length(4),
                            Constraint::Length(3),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[1]);

                let progress_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Ratio(1, 4),
                            Constraint::Ratio(2, 4),
                            Constraint::Ratio(1, 4),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[0]);

                self.game.draw_side(f, progress_chunks[0], AiSide::For);
                self.game.draw_progress(f, progress_chunks[1]);
                self.game.draw_side(f, progress_chunks[2], AiSide::Against);
                self.game.draw_captures(f, chunks[1]);
                self.game.draw_race(f, chunks[2]);
            }
        }

        // let other_block = Block::default().title("Others");
        // f.render_widget(other_block, chunks[1]);

        // f.render_widget(block, chunks[0]);
        // let chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .margin(2)
        //     .constraints(
        //         [
        //             Constraint::Length(1),
        //             Constraint::Length(3),
        //             Constraint::Min(1),
        //         ]
        //         .as_ref(),
        //     )
        //     .split(f.size());

        // let (msg, style) = match self.input_mode {
        //     InputMode::Normal => (
        //         vec![
        //             Span::raw("Press "),
        //             Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        //             Span::raw(" to exit, "),
        //             Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
        //             Span::raw(" to start editing."),
        //         ],
        //         Style::default().add_modifier(Modifier::RAPID_BLINK),
        //     ),
        //     InputMode::Editing => (
        //         vec![
        //             Span::raw("Press "),
        //             Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        //             Span::raw(" to stop editing, "),
        //             Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        //             Span::raw(" to record the message"),
        //         ],
        //         Style::default(),
        //     ),
        // };

        // let mut text = Text::from(Spans::from(msg));
        // text.patch_style(style);
        // let help_message = Paragraph::new(text);
        // f.render_widget(help_message, chunks[0]);

        // let input = Paragraph::new(self.input.as_ref())
        //     .style(match self.input_mode {
        //         InputMode::Normal => Style::default(),
        //         InputMode::Editing => Style::default().fg(Color::Yellow),
        //     })
        //     .block(Block::default().borders(Borders::ALL).title("Input"));
        // f.render_widget(input, chunks[1]);
        // match self.input_mode {
        //     InputMode::Normal =>
        //         // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        //         {}

        //     InputMode::Editing => {
        //         // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        //         f.set_cursor(
        //             // Put cursor past the end of the input text
        //             chunks[1].x + self.input.width() as u16 + 1,
        //             // Move one line down, from the border to the input line
        //             chunks[1].y + 1,
        //         )
        //     }
        // }

        // let messages: Vec<ListItem> = self
        //     .messages
        //     .iter()
        //     .enumerate()
        //     .map(|(i, m)| {
        //         let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
        //         ListItem::new(content)
        //     })
        //     .collect();
        // let messages =
        //     List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        // f.render_widget(messages, chunks[2]);
    }
}

impl App<'_> {
    pub fn on_tick(&mut self) {
        if let Some(race) = &self.game.race {
            if race.is_finished() {
                self.game.race = None;
            }
        }
    }

    pub fn on_key(&mut self, code: KeyCode) {
        match &self.state {
            AppState::PlayerInput(ai_side) => match code {
                KeyCode::Enter => match ai_side {
                    AiSide::For => {
                        if let Some(_) = self.game.for_ai {
                            self.state = AppState::PlayerInput(AiSide::Against)
                        }
                    }
                    AiSide::Against => {
                        if let Some(_) = self.game.against_ai {
                            let mut rng = rand::thread_rng();
                            self.vpn_positions = Some((
                                vpn_position(rng.borrow_mut(), AiSide::For),
                                vpn_position(rng.borrow_mut(), AiSide::Against),
                            ));
                            self.state = AppState::VPNPositions
                        }
                    }
                },
                KeyCode::Char(c) => {
                    if let Some(d) = c.to_digit(10) {
                        let d = d as u8;
                        if MIN_PLAYERS <= d && d <= MAX_PLAYERS {
                            match ai_side {
                                AiSide::For => {
                                    self.game.for_ai = Some(Side::new(d as u8, AiSide::For))
                                }
                                AiSide::Against => {
                                    self.game.against_ai = Some(Side::new(d as u8, AiSide::Against))
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            AppState::VPNPositions => match code {
                KeyCode::Enter => {
                    let mut rng = rand::thread_rng();
                    self.capture_positions = Some((
                        capture_position(rng.borrow_mut(), AiSide::For),
                        capture_position(rng.borrow_mut(), AiSide::Against),
                        capture_position(rng.clone().borrow_mut(), rng.gen()),
                    ));
                    self.state = AppState::CapturePositions;
                }
                _ => {}
            },
            AppState::CapturePositions => match code {
                KeyCode::Enter => self.state = AppState::Play,
                _ => {}
            },
            _ => {}
        }
    }
}
