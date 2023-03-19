use std::ops::RangeInclusive;

use rand::{distributions::Standard, prelude::Distribution, Rng};

use super::{capture::Capture, race::Race, side::Side};

pub const ROW_RANGE: RangeInclusive<u8> = 1..=7;
pub const COL_RANGE: RangeInclusive<u8> = 1..=11;

pub const DATABASE_POINTS: u32 = 7;

const COL_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// Every end of turn test this
pub const RACE_PROB: f64 = 1.0 / 5.0;

pub const CHALLENGE_POINTS: u32 = 3;

// Per team
pub const MIN_PLAYERS: u8 = 1;
pub const MAX_PLAYERS: u8 = 3;

#[derive(Clone, PartialEq)]
pub enum AiSide {
    For,
    Against,
}

impl AiSide {
    pub fn switch(&self) -> AiSide {
        match self {
            Self::For => AiSide::Against,
            Self::Against => AiSide::For,
        }
    }
}

impl Distribution<AiSide> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AiSide {
        match rng.gen_range(0..=1) {
            0 => AiSide::For,
            1 => AiSide::Against,
            _ => panic!("Should never happen!"),
        }
    }
}

pub struct Game {
    pub for_ai: Option<Side>,
    pub against_ai: Option<Side>,
    pub center_capture: Option<Capture>,
    pub race: Option<Race>,
    pub turn: AiSide,
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        Game {
            for_ai: None,
            against_ai: None,
            center_capture: None,
            race: None,
            turn: rng.gen(),
        }
    }

    pub fn get_turn(&mut self) -> Option<&mut Side> {
        match self.turn {
            AiSide::For => self.for_ai.as_mut(),
            AiSide::Against => self.against_ai.as_mut(),
        }
    }
}

#[derive(PartialEq)]
pub struct Position {
    pub(super) row: u8,
    pub(super) col: u8,
}

impl Distribution<Position> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        Position {
            row: rng.gen_range(ROW_RANGE),
            col: rng.gen_range(COL_RANGE),
        }
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            COL_ALPHABET.chars().nth(self.col as usize - 1).unwrap(),
            self.row.to_string()
        )
    }
}
