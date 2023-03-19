use std::ops::RangeInclusive;

use rand::{distributions::Standard, prelude::Distribution, Rng};

use super::{capture::Capture, race::Race, side::Side};

pub const ROW_RANGE: RangeInclusive<u8> = 1..=7;
pub const COL_RANGE: RangeInclusive<u8> = 1..=11;

const COL_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// Per team
pub const MIN_PLAYERS: u8 = 1;
pub const MAX_PLAYERS: u8 = 3;

#[derive(Clone)]
pub enum AiSide {
    For,
    Against,
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
            center_capture: Some(Capture::new(AiSide::Against)),
            race: Some(Race::new()),
            turn: rng.gen(),
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
