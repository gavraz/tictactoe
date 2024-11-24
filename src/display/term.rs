use super::Display;
use crate::game::{Cell, Status, MoveError, Outcome, Player};
use std::fmt::Formatter;

pub struct TerminalDisplay {}

impl TerminalDisplay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for TerminalDisplay {
    fn on_change(&mut self, status: std::result::Result<Status, MoveError>) {
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

    fn draw(&mut self, state: [[Cell; 3]; 3]) {
        println!("┌───┬───┬───┐");
        for (i, row) in state.iter().enumerate() {
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
    
    fn on_input(&mut self, res: &std::result::Result<crate::input::Result, std::num::ParseIntError>) {
        match res {
            Ok(_) => {},
            Err(e) => println!("Incorrect input{e}\nChoose a position (Format: i,j):"),
        }
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

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}
