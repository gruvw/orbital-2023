use rand::{distributions::Standard, prelude::Distribution, rngs::ThreadRng, Rng};

use super::game::{AiSide, Position, ROW_RANGE};

impl AiSide {
    pub fn vpn_column(&self) -> u8 {
        match self {
            AiSide::For => 3,
            AiSide::Against => 9,
        }
    }
}

pub fn vpn_position(rng: &mut ThreadRng, ai_side: AiSide) -> Position {
    Position {
        row: rng.gen_range(ROW_RANGE),
        col: ai_side.vpn_column(),
    }
}
