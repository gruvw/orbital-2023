use rand::Rng;

use super::{capture::Capture, game::AiSide};

pub const TOTAL_PROGRESS: u32 = 31;

pub struct Side {
    pub nb_players: u8,
    pub progress: u32,
    pub nb_rounds: u32,
    pub capture: Capture,
}

impl Side {
    pub fn new(nb_players: u8, ai_side: AiSide) -> Side {
        Side {
            nb_players,
            nb_rounds: 0,
            progress: 0,
            capture: Capture::new(ai_side),
        }
    }
}
