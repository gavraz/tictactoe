pub mod term;
pub mod tui;

pub trait Display {
    fn on_move(&mut self, status: std::result::Result<super::GameStatus, super::game::MoveError>);
    fn draw_board(&self, state: [[super::game::Cell; 3]; 3]);
    fn message(&self, msg: impl std::fmt::Display);
}
