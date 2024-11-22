use super::Display;
use super::super::{GameStatus, game::MoveError};
use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

pub struct TuiDisplay {
    term : ratatui::DefaultTerminal
}

impl TuiDisplay {
    pub fn new() -> Self {
        TuiDisplay{term:ratatui::init()} 
    }
}

impl Display for TuiDisplay {
    fn on_move(&mut self, status: std::result::Result<GameStatus, MoveError>) {
        self.term.draw(draw).expect("failed to draw frame");
    }

    fn draw_board(&self, state: [[crate::game::Cell; 3]; 3]) {
        
    }
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}