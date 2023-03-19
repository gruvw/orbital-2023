pub mod capture;
pub mod finished;
pub mod keys;
pub mod progress;
pub mod race;
pub mod setup;
pub mod side;
pub mod title;

use tui::style::Color;

use super::data::game::AiSide;

impl AiSide {
    pub fn color(&self) -> Color {
        match self {
            AiSide::For => Color::Cyan,
            AiSide::Against => Color::Red,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            AiSide::For => "For AI",
            AiSide::Against => "Against AI",
        }
    }
}
