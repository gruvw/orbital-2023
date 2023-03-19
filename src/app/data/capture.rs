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
    count: u32,
    pub ai_side: AiSide,
}

impl Capture {
    pub fn new(ai_side: AiSide) -> Capture {
        Capture { count: 0, ai_side }
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn add(&mut self) {
        self.count += 1;
    }

    pub fn remove(&mut self) {
        if self.count >= 1 {
            self.count -= 1;
        }
    }
}

impl AiSide {
    fn pos_inside(&self, pos: &Position) -> bool {
        match self {
            AiSide::For => pos.col < 6,
            AiSide::Against => pos.col > 6,
        }
    }
}

pub fn capture_position(rng: &mut ThreadRng, ai_side: AiSide) -> Position {
    loop {
        let position = rng.gen();
        if !ILLEGAL_CAPTURE_POSITIONS.contains(&position)
            && (position.col < 5 || position.col > 7)
            && ai_side.pos_inside(&position)
        {
            return position;
        }
    }
}
