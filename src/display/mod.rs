pub mod term;
pub mod tui;

pub trait Display {
    fn on_move(&mut self, result: super::game::GameResult);
    fn draw_board(&self, state: [[super::game::Cell; 3]; 3]);
}