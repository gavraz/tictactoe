mod game;
mod display;
mod input;

use crate::game::{Game, GameResult};

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

        let move_result = game.apply(i, j);
        display.on_move(move_result);

        match move_result {
            GameResult::Tie => { break; }
            GameResult::Winner(_) => { break; }
            _ => ()
        };
    }

    display.draw_board(game.state());
}