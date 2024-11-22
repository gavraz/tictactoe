mod game;
mod display;
mod input;

use game::GameStatus;
use crate::game::Game;

fn main() {
    let mut game: Game = Game::new();
    let display: &mut dyn display::Display = &mut display::term::TerminalDisplay::new();
    let input: &mut dyn input::Input = &mut input::TerminalInput{};

    loop {
        display.draw_board(game.state());

        let (i,j) = match input.get() {
            input::Result::Position(i, j) => (i,j),
            input::Result::Exit => return,
        };

        let status = game.apply(i, j);
        display.on_move(status);

        match status {
            Ok(game::GameStatus::Ended(_)) => break,
            _ => {},
        };
    }

    display.draw_board(game.state());
}