use super::Display;
use crate::game::{Cell, GameStatus, MoveError, Outcome, Player};
use std::fmt::Formatter;

pub struct TerminalDisplay {}

impl TerminalDisplay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for TerminalDisplay {
    fn on_move(&mut self, status: std::result::Result<GameStatus, MoveError>) {
        match status {
            Ok(status) => match status {
                GameStatus::Playing(player) => {
                    println!("Current player: {player}");
                }
                GameStatus::Ended(Outcome::Tie) => {
                    println!("Game result: Tie");
                }
                GameStatus::Ended(Outcome::Win(player)) => {
                    println!("Game result: {player} wins");
                }
            },
            Err(e) => match e {
                MoveError::AlreadyOccupied => println!("Incorrect move: cell already occupied"),
                MoveError::OutOfBounds => println!("Incorrect move: input is out of bounds"),
            },
        }
    }

    fn draw_board(&self, state: [[Cell; 3]; 3]) {
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

    fn message(&self, msg: impl std::fmt::Display) {
        println!("{msg}");
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
