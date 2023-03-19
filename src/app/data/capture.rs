use rand::{rngs::ThreadRng, Rng};

use super::game::{AiSide, Position};

pub const ILLEGAL_CAPTURE_POSITIONS: &'static [Position] = &[
    Position { row: 4, col: 1 },  // Base
    Position { row: 4, col: 11 }, // Base
    Position { row: 3, col: 6 },  // Middle
    Position { row: 4, col: 6 },  // Middle
    Position { row: 5, col: 6 },  // Middle
];

pub struct Capture {
    count: u16,
    pub ai_side: AiSide,
}

impl Capture {
    pub fn new(ai_side: AiSide) -> Capture {
        Capture { count: 2, ai_side }
    }

    pub fn count(&self) -> u16 {
        self.count
    }

    fn add(&mut self) {
        self.count += 1;
    }

    fn remove(&mut self) {
        self.count -= 1;
    }
}

pub fn capture_position(rng: &mut ThreadRng) -> Position {
    loop {
        let position = rng.gen();
        if !ILLEGAL_CAPTURE_POSITIONS.contains(&position) {
            return position;
        }
    }
}
