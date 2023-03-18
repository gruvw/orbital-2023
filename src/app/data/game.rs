use super::{capture::Capture, side::Side};

pub enum AiSide {
    For,
    Against,
}

pub struct Game {
    pub for_ai: Side,
    pub against_ai: Side,
    pub center_capture: Option<Capture>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            for_ai: Side::new(AiSide::For),
            against_ai: Side::new(AiSide::Against),
            center_capture: Some(Capture::new(AiSide::Against)),
        }
    }
}
