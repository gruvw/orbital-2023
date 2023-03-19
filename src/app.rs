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

use self::{
    data::{
        capture::capture_position,
        game::{AiSide, Game, Position, MAX_PLAYERS, MIN_PLAYERS, RACE_PROB},
        race::{Race, RACE_POINTS},
        side::Side,
        vpn::vpn_position,
    },
    widgets::title::Title,
};

pub trait Drawable {
    fn draw<B: Backend>(&self, app: &App, f: &mut Frame<B>, rect: Rect);
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
                self.draw_setup(f, chunks[1], ai_side);
            }
            AppState::VPNPositions | AppState::CapturePositions => {
                self.draw_setup(f, chunks[1], AiSide::For);
            }
            AppState::Play => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(10)
                    .constraints(
                        [
                            Constraint::Length(9), // Progress
                            Constraint::Length(4), // Captures
                            Constraint::Length(3), // Race
                            Constraint::Length(9), // Keys
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
                self.game.draw_keys(f, chunks[3]);
            }
        }
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
        let mut rng = rand::thread_rng();
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
            AppState::Play => match code {
                KeyCode::Esc => self.should_quit = true,
                KeyCode::Tab => {
                    // Change turn
                    if let Some(ref mut side) = self.game.get_turn() {
                        side.nb_rounds += 1;
                    }
                    self.game.turn = self.game.turn.switch();

                    if let None = self.game.race {
                        if rng.gen_bool(RACE_PROB) {
                            self.game.race = Some(Race::new());
                        }
                    }
                }
                KeyCode::Char(c) => match c {
                    '1' => {
                        if let Some(ref mut for_ai) = self.game.for_ai {
                            for_ai.progress += RACE_POINTS;
                            self.game.race = None;
                        }
                    }
                    '0' => {
                        if let Some(ref mut against_ai) = self.game.against_ai {
                            against_ai.progress += RACE_POINTS;
                            self.game.race = None;
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
        }
    }
}
