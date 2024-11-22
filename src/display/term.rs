use super::Display;
use std::fmt::Formatter;
use crate::game::{GameStatus, Player, Cell, Outcome, MoveError};

pub struct TerminalDisplay {}

impl TerminalDisplay {
    pub fn new() -> Self {
        TerminalDisplay {}
    }
}

impl Display for TerminalDisplay {
    fn on_move(&mut self, status: std::result::Result<GameStatus, MoveError>) {
        match status {
            Ok(status) => {
                match status {
                    GameStatus::Playing(player) => { println!("Current player: {}", player) }
                    GameStatus::Ended(Outcome::Tie) => {
                        println!("Game result: Tie");
                    }
                    GameStatus::Ended(Outcome::Win(player)) => {
                        println!("Game result: {} wins", player);
                    }
                }
            }
            Err(e) => {
                match e {
                    MoveError::AlreadyOccupied => println!("Incorrect move: cell already occupied"),
                    MoveError::OutOfBounds => println!("Incorrect move: input is out of bounds"),
                }
            },
        }
    }

    fn draw_board(&self, state: [[Cell; 3]; 3]) {
        println!("┌───┬───┬───┐");
        for (i, row) in state.iter().enumerate() {
            print!("│");
            for c in row.iter() {
                print!(" {} │", c);
            }
            println!();
            if i < 2 {
                println!("├───┼───┼───┤");
            }
        }
        println!("└───┴───┴───┘");
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Filled(Player::X) => write!(f, "X"),
            Cell::Filled(Player::O) => write!(f, "O"),
            Cell::Empty => write!(f, " "),
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}