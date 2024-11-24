mod display;
mod game;
mod input;

use crate::display::Display;
use crate::game::Game;
use game::Status;
use input::Input;

fn main() {
    let mut game = Game::new();
    let mut display = display::tui::TuiDisplay::new();
    let mut input = input::CrossTermInput::new();
    // let mut display = display::term::TerminalDisplay::new();
    // let mut input = input::TerminalInput::new();

    loop {
        display.draw(game.state());

        let input = input.get();

        display.on_input(&input);
        let (i,j) = match input {
            Ok(input::Result::Position(i, j)) => (i,j),
            Ok(input::Result::Exit) => break,
            _ => continue,
        };
        let status = game.apply(i, j);
        display.on_change(status);

        match status {
            Ok(game::Status::Ended(_)) => break,
            _ => {}
        };
    }

    display.draw(game.state());
}
