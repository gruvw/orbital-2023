use rand::Rng;

use super::{capture::Capture, game::AiSide};

pub const TOTAL_PROGRESS: u32 = 31;

pub struct Side {
    pub nb_players: u8,
    pub progress: u32,
    pub capture: Capture,
}

impl Side {
    pub fn new(nb_players: u8, ai_side: AiSide) -> Side {
        let mut rng = rand::thread_rng();
        Side {
            nb_players,
            progress: rng.gen_range((TOTAL_PROGRESS / 2)..TOTAL_PROGRESS),
            capture: Capture::new(ai_side),
        }
    }
}
