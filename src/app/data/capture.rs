use super::game::AiSide;

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
