mod data;
mod widgets;

use crossterm::event::KeyCode;
use itertools::Itertools;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use self::{
    data::game::{AiSide, Game},
    widgets::title::Title,
};

pub trait Drawable {
    fn draw<B: Backend>(&self, app: &App, f: &mut Frame<B>, rect: Rect);
}
enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
pub struct App<'a> {
    title: Title<'a>,
    game: Game,
    /// Current value of the input box
    // input: String,
    // /// Current input mode
    // input_mode: InputMode,
    // /// History of recorded messages
    // messages: Vec<String>,
    pub should_quit: bool,
}

impl App<'_> {
    pub fn new() -> App<'static> {
        let game = Game::new();
        App {
            title: Title::new("Welcome to the best Game CyberConnect!"),
            game,
            // input: String::new(),
            // input_mode: InputMode::Normal,
            // messages: Vec::new(),
            should_quit: false,
        }
    }
}

impl App<'_> {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Length(4),
                    Constraint::Length(4),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(f.size());

        let progress_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 4),
                    Constraint::Ratio(2, 4),
                    Constraint::Ratio(1, 4),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        self.title.draw(self, f, chunks[0]);
        self.game.draw_side(f, progress_chunks[0], AiSide::For);
        self.game.draw_progress(f, progress_chunks[1]);
        self.game.draw_side(f, progress_chunks[2], AiSide::Against);
        self.game.draw_captures(f, chunks[2]);
        // self.game.for_ai.draw(self, f, progress_chunks[0]);
        // self.game.against_ai.draw(self, f, progress_chunks[2]);

        // let other_block = Block::default().title("Others");
        // f.render_widget(other_block, chunks[1]);

        // f.render_widget(block, chunks[0]);
        // let chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .margin(2)
        //     .constraints(
        //         [
        //             Constraint::Length(1),
        //             Constraint::Length(3),
        //             Constraint::Min(1),
        //         ]
        //         .as_ref(),
        //     )
        //     .split(f.size());

        // let (msg, style) = match self.input_mode {
        //     InputMode::Normal => (
        //         vec![
        //             Span::raw("Press "),
        //             Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        //             Span::raw(" to exit, "),
        //             Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
        //             Span::raw(" to start editing."),
        //         ],
        //         Style::default().add_modifier(Modifier::RAPID_BLINK),
        //     ),
        //     InputMode::Editing => (
        //         vec![
        //             Span::raw("Press "),
        //             Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        //             Span::raw(" to stop editing, "),
        //             Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        //             Span::raw(" to record the message"),
        //         ],
        //         Style::default(),
        //     ),
        // };

        // let mut text = Text::from(Spans::from(msg));
        // text.patch_style(style);
        // let help_message = Paragraph::new(text);
        // f.render_widget(help_message, chunks[0]);

        // let input = Paragraph::new(self.input.as_ref())
        //     .style(match self.input_mode {
        //         InputMode::Normal => Style::default(),
        //         InputMode::Editing => Style::default().fg(Color::Yellow),
        //     })
        //     .block(Block::default().borders(Borders::ALL).title("Input"));
        // f.render_widget(input, chunks[1]);
        // match self.input_mode {
        //     InputMode::Normal =>
        //         // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        //         {}

        //     InputMode::Editing => {
        //         // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        //         f.set_cursor(
        //             // Put cursor past the end of the input text
        //             chunks[1].x + self.input.width() as u16 + 1,
        //             // Move one line down, from the border to the input line
        //             chunks[1].y + 1,
        //         )
        //     }
        // }

        // let messages: Vec<ListItem> = self
        //     .messages
        //     .iter()
        //     .enumerate()
        //     .map(|(i, m)| {
        //         let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
        //         ListItem::new(content)
        //     })
        //     .collect();
        // let messages =
        //     List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        // f.render_widget(messages, chunks[2]);
    }
}

impl App<'_> {
    pub fn on_tick(&mut self) {}

    pub fn on_key(&mut self, code: KeyCode) {
        // match self.input_mode {
        //     InputMode::Normal => match code {
        //         KeyCode::Char('e') => {
        //             self.input_mode = InputMode::Editing;
        //         }
        //         _ => {}
        //     },
        //     InputMode::Editing => match code {
        //         KeyCode::Enter => {
        //             self.messages.push(self.input.drain(..).collect());
        //         }
        //         KeyCode::Char(c) => {
        //             self.input.push(c);
        //         }
        //         KeyCode::Backspace => {
        //             self.input.pop();
        //         }
        //         KeyCode::Esc => {
        //             self.input_mode = InputMode::Normal;
        //         }
        //         _ => {}
        //     },
        // }
    }
}
