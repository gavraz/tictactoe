use crate::game::State;

pub mod term;
pub mod tui;

pub trait Display {
    fn update(&mut self, state: State);
    fn on_move(&mut self, status: std::result::Result<super::Status, super::game::MoveError>);
    fn draw(&mut self);
    fn on_input(&mut self, res: &std::result::Result<super::input::Result, std::num::ParseIntError>);
}
