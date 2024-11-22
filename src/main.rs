mod display;
mod game;
mod input;

use crate::display::Display;
use crate::game::Game;
use game::GameStatus;
use input::Input;

fn main() {
    let mut game: Game = Game::new();
    let mut display = display::term::TerminalDisplay::new();
    let mut input = input::TerminalInput::new();

    loop {
        display.draw_board(game.state());

        display.message("Choose a position (Format: i,j):");
        let (i, j) = match input.get() {
            Ok(input::Result::Position(i, j)) => (i, j),
            Ok(input::Result::Exit) => return,
            Err(e) => {
                display.message(format!("Incorrect input: {}", e));
                continue;
            }
        };

        let status = game.apply(i, j);
        display.on_move(status);

        match status {
            Ok(game::GameStatus::Ended(_)) => break,
            _ => {}
        };
    }

    display.draw_board(game.state());
}
