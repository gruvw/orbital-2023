use super::{capture::Capture, game::AiSide};

pub const TOTAL_PROGRESS: u32 = 50;

pub struct Side {
    pub nb_players: u8,
    progress: u32,
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

    pub fn advance(&mut self, value: u32) {
        if self.progress + value > TOTAL_PROGRESS {
            self.progress = TOTAL_PROGRESS;
        } else {
            self.progress += value;
        }
    }

    pub fn retreat(&mut self, value: u32) {
        if self.progress < value {
            self.progress = 0;
        } else {
            self.progress -= value;
        }
    }

    pub fn progress(&self) -> u32 {
        self.progress
    }
}
