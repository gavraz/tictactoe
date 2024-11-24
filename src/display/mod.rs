pub mod term;
pub mod tui;

pub trait Display {
    fn on_change(&mut self, status: std::result::Result<super::Status, super::game::MoveError>);
    fn draw(&mut self, state: [[super::game::Cell; 3]; 3]);
    fn on_input(&mut self, res: &std::result::Result<super::input::Result, std::num::ParseIntError>);
}
