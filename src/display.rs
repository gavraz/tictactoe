use std::fmt::Formatter;
use crate::game::{GameResult, Player};
use crate::game::Cell;

pub struct Display {}

impl Display {
    pub fn new() -> Display {
        Display {}
    }

    pub fn on_move(&self, result: GameResult) {
        match result {
            GameResult::Playing(player) => { println!("Current player: {}", player) }
            GameResult::Tie => {
                println!("Game result: Tie");
            }
            GameResult::Winner(player) => {
                println!("Game result: {} wins", player);
            }
            GameResult::AlreadyTaken => { println!("Already taken.") }
            GameResult::InvalidMove => { println!("Invalid move.") }
        }
    }

    pub fn draw_board(&self, state: [[Cell; 3]; 3]) {
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
            Cell::PlayerX => write!(f, "X"),
            Cell::PlayerO => write!(f, "O"),
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