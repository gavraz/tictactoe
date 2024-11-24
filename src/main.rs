mod display;
mod game;
mod input;

mod ratatui;
use ratatui as controller;
// mod term;
// use term as controller;

use display::Display;
use game::{Game, Status};
use input::Input;

fn main() {
    let mut game = Game::new();
    let mut display = controller::display::Display::new(game.state());
    let mut input = controller::input::Input::new();

    display.draw();

    loop {
        let input_res = input.get();

        display.on_input(&input_res);
        let (i, j) = match input_res {
            Ok(input::Result::Position(i, j)) => (i, j),
            Ok(input::Result::Exit) => return,
            _ => continue,
        };
        let status = game.apply(i, j);
        display.on_move(status);

        match status {
            Ok(game::Status::Ended(_)) => break,
            _ => {}
        };

        display.update(game.state());
        display.draw();
    }

    display.update(game.state());
    display.draw();

    input.wait_exit();
}
