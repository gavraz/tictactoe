use crate::game::{Cell, MoveError, Outcome, Player, State, Status};
use std::fmt::Formatter;
use super::super::display;

pub struct Display {
    state: State,
}

impl Display {
    pub fn new(state: State) -> Self {
        Self { state }
    }
}

impl display::Display for Display {
    fn on_move(&mut self, status: std::result::Result<Status, MoveError>) {
        match status {
            Ok(status) => match status {
                Status::Playing(player) => {
                    println!("Current player: {player}");
                }
                Status::Ended(Outcome::Tie) => {
                    println!("Game result: Tie");
                }
                Status::Ended(Outcome::Win(player)) => {
                    println!("Game result: {player} wins");
                }
            },
            Err(e) => match e {
                MoveError::AlreadyOccupied => println!("Incorrect move: cell already occupied"),
                MoveError::OutOfBounds => println!("Incorrect move: input is out of bounds"),
            },
        }
    }

    fn draw(&mut self) {
        println!("┌───┬───┬───┐");
        for (i, row) in self.state.board.iter().enumerate() {
            print!("│");
            for c in row {
                print!(" {c} │");
            }
            println!();
            if i < 2 {
                println!("├───┼───┼───┤");
            }
        }
        println!("└───┴───┴───┘");
    }

    fn on_input(
        &mut self,
        res: &std::result::Result<crate::input::Result, std::num::ParseIntError>,
    ) {
        match res {
            Ok(_) => {}
            Err(e) => println!("Incorrect input{e}\nChoose a position (Format: i,j):"),
        }
    }

    fn update(&mut self, state: State) {
        self.state = state;
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Filled(Player::X) => write!(f, "X"),
            Self::Filled(Player::O) => write!(f, "O"),
            Self::Empty => write!(f, " "),
        }
    }
}
