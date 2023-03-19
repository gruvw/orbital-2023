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
        capture::{capture_position, Capture, CAPTURE_POINTS, CENTER_CAPTURE_MULTIPLIER},
        game::{
            AiSide, Game, Position, CHALLENGE_POINTS, DATABASE_POINTS, MAX_PLAYERS, MIN_PLAYERS,
            RACE_PROB,
        },
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
    Finished(AiSide),
    Challenge(&'static str),
}

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
            title: Title::new(
                "Welcome to CyberConnect!    Fight against the other team to control the AI \"Lucy\".    You have many funny mechanics to discover along the way :)    Have Fun !",
            ),
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
            .constraints([Constraint::Length(2), Constraint::Max(80)].as_ref())
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
                            Constraint::Length(1),
                            Constraint::Length(4), // Captures
                            Constraint::Length(1),
                            Constraint::Length(3), // Race
                            Constraint::Length(1),
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
                            Constraint::Length(1),
                            Constraint::Ratio(2, 4),
                            Constraint::Length(1),
                            Constraint::Ratio(1, 4),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[0]);

                self.game.draw_side(f, progress_chunks[0], AiSide::For);
                self.game.draw_progress(f, progress_chunks[2]);
                self.game.draw_side(f, progress_chunks[4], AiSide::Against);
                self.game.draw_captures(f, chunks[2]);
                self.game.draw_race(f, chunks[4]);
                self.game.draw_keys(f, chunks[6]);
            }
            AppState::Challenge(text) => {
                self.draw_challenge(f, chunks[1], text);
            }
            AppState::Finished(ai_side) => {
                self.draw_finished(f, chunks[1], ai_side);
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

        if let Some(ref mut center) = self.game.center_capture {
            if center.count() == 0 {
                self.game.center_capture = None;
            }
        }

        // Winning condition
        if let Some(ref mut for_ai) = self.game.for_ai {
            if for_ai.has_won() {
                self.state = AppState::Finished(AiSide::For)
            }
        }

        if let Some(ref mut against_ai) = self.game.against_ai {
            if against_ai.has_won() {
                self.state = AppState::Finished(AiSide::Against)
            }
        }
    }

    pub fn on_key(&mut self, code: KeyCode) {
        if let KeyCode::Esc = code {
            self.should_quit = true;
            return;
        };

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
                KeyCode::Tab => {
                    // Change turn (SWITCH TEAM)
                    if let Some(ref mut side) = self.game.get_turn() {
                        side.nb_rounds += 1;
                    }
                    self.game.turn = self.game.turn.switch();

                    // Count capture points
                    let mut center_captured = 0;
                    if let Some(center) = &self.game.center_capture {
                        if center.ai_side == self.game.turn {
                            center_captured = center.count();
                        }
                    }
                    if let Some(ref mut side) = self.game.get_turn() {
                        side.advance(
                            CAPTURE_POINTS
                                * (side.capture.count()
                                    + (CENTER_CAPTURE_MULTIPLIER * center_captured as f32).round()
                                        as u32),
                        );
                    }

                    if let None = self.game.race {
                        if rng.gen_bool(RACE_PROB) {
                            self.game.race = Some(Race::new());
                        }
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(ref mut for_ai) = self.game.for_ai {
                        if let Some(ref mut against_ai) = self.game.against_ai {
                            match c {
                                '1' => {
                                    if let Some(_) = self.game.race {
                                        for_ai.advance(RACE_POINTS);
                                        self.game.race = None;
                                    }
                                }
                                '0' => {
                                    if let Some(_) = self.game.race {
                                        against_ai.advance(RACE_POINTS);
                                        self.game.race = None;
                                    }
                                }

                                '5' => for_ai.retreat(1),
                                '7' => against_ai.retreat(1),

                                'i' => for_ai.advance(DATABASE_POINTS),
                                'd' => against_ai.advance(DATABASE_POINTS),

                                'e' => for_ai.capture.add(),
                                'u' => for_ai.capture.remove(),
                                't' => against_ai.capture.add(),
                                'h' => against_ai.capture.remove(),

                                'a' => {
                                    if let Some(ref mut center) = self.game.center_capture {
                                        match center.ai_side {
                                            AiSide::For => center.add(),
                                            AiSide::Against => {
                                                *center = Capture::new(AiSide::For);
                                                center.add();
                                            }
                                        }
                                    } else {
                                        let mut center = Capture::new(AiSide::For);
                                        center.add();
                                        self.game.center_capture = Some(center);
                                    }
                                }
                                'o' => {
                                    if let Some(ref mut center) = self.game.center_capture {
                                        if let AiSide::For = center.ai_side {
                                            center.remove()
                                        }
                                    }
                                }
                                's' => {
                                    if let Some(ref mut center) = self.game.center_capture {
                                        match center.ai_side {
                                            AiSide::Against => center.add(),
                                            AiSide::For => {
                                                *center = Capture::new(AiSide::Against);
                                                center.add();
                                            }
                                        }
                                    } else {
                                        let mut center = Capture::new(AiSide::Against);
                                        center.add();
                                        self.game.center_capture = Some(center);
                                    }
                                }
                                'n' => {
                                    if let Some(ref mut center) = self.game.center_capture {
                                        if let AiSide::Against = center.ai_side {
                                            center.remove()
                                        }
                                    }
                                }

                                'f' => self.state = AppState::Challenge(self.game.pick_challenge()),
                                _ => {}
                            };
                        }
                        // WHY ARE YOU EVEN READING THIS ? IT IS HORRIBLE...
                    }
                }
                _ => {}
            },
            AppState::Challenge(text) => {
                match code {
                    KeyCode::Char('f') => {
                        // Pick a different challenge
                        self.state = AppState::Challenge(loop {
                            let challenge = self.game.pick_challenge();
                            if challenge != *text {
                                break challenge;
                            }
                        })
                    }

                    KeyCode::Char('q') => self.state = AppState::Play,

                    KeyCode::Char('1') => {
                        if let Some(ref mut for_ai) = self.game.for_ai {
                            for_ai.advance(CHALLENGE_POINTS)
                        }
                        self.state = AppState::Play
                    }
                    KeyCode::Char('0') => {
                        if let Some(ref mut against_ai) = self.game.against_ai {
                            against_ai.advance(CHALLENGE_POINTS)
                        }
                        self.state = AppState::Play
                    }
                    _ => {}
                }
            }
            AppState::Finished(_) => {}
        }
    }
}
