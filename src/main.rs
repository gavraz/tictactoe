mod game;
mod display;
mod input;

use crate::game::{Game, GameResult};

fn main() {
    let mut game: Game = Game::new();
    let display: display::Display = display::Display::new();

    loop {
        display.draw_board(game.state());

        let (i, j) = input::input();

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