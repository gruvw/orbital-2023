use std::ops::RangeInclusive;

use rand::{distributions::Standard, prelude::Distribution, rngs::ThreadRng, Rng};

use super::{capture::Capture, race::Race, side::Side};

pub const ROW_RANGE: RangeInclusive<u8> = 1..=7;
pub const COL_RANGE: RangeInclusive<u8> = 1..=11;

const COL_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub enum AiSide {
    For,
    Against,
}

pub struct Game {
    pub for_ai: Side,
    pub against_ai: Side,
    pub center_capture: Option<Capture>,
    pub race: Option<Race>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            for_ai: Side::new(AiSide::For),
            against_ai: Side::new(AiSide::Against),
            center_capture: Some(Capture::new(AiSide::Against)),
            race: Some(Race::new()),
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
            COL_ALPHABET.chars().nth(self.col as usize).unwrap(),
            self.row.to_string()
        )
    }
}
