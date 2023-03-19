use std::time::{Duration, Instant};

use rand::Rng;

use super::game::Position;

pub const SECS_IN_MIN: u64 = 60;
const RACE_DURATION: Duration = Duration::from_secs(2 * SECS_IN_MIN);
pub const RACE_POINTS: u32 = 5;

pub struct Race {
    start: Instant,
    pub position: Position,
}

impl Race {
    pub fn new() -> Race {
        Race {
            start: Instant::now(),
            position: rand::thread_rng().gen(),
        }
    }

    pub fn remaining_time(&self) -> chrono::Duration {
        let took = Instant::now().duration_since(self.start);
        if RACE_DURATION <= took {
            return chrono::Duration::zero();
        }
        chrono::Duration::from_std(RACE_DURATION - took).unwrap()
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_time().is_zero()
    }
}
