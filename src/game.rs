use crate::game::Cell::Empty;
use crate::game::GameStatus::{Ended, Playing};
use crate::game::Outcome::{Tie, Win};
use crate::game::Player::{O, X};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Filled(Player),
    Empty,
}

impl Cell {
    fn player(self) -> Option<Player> {
        match self {
            Self::Filled(player) => Some(player),
            Empty => None,
        }
    }
}

impl Player {
    fn opposite(self) -> Self {
        match self {
            X => O,
            O => X,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Win(Player),
    Tie,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameStatus {
    Playing(Player),
    Ended(Outcome),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveError {
    AlreadyOccupied,
    OutOfBounds,
}

const BOARD_SIZE: usize = 3;
pub struct Game {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
    moves: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [[Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: X,
            moves: 0,
        }
    }

    pub fn apply(&mut self, i: usize, j: usize) -> std::result::Result<GameStatus, MoveError> {
        if i > 2 || j > 2 {
            return Err(MoveError::OutOfBounds);
        }

        if self.board[i][j] != Empty {
            return Err(MoveError::AlreadyOccupied);
        }

        self.board[i][j] = Cell::Filled(self.current_player);
        self.moves += 1;

        Ok(self.check())
    }

    fn check(&mut self) -> GameStatus {
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            match self.board[0][0].player() {
                Some(X) => return Ended(Win(X)),
                Some(O) => return Ended(Win(O)),
                None => (),
            }
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            match self.board[0][2].player() {
                Some(X) => return Ended(Win(X)),
                Some(O) => return Ended(Win(O)),
                None => (),
            }
        }

        for row in self.board {
            if row[0] == row[1] && row[1] == row[2] {
                match row[0].player() {
                    Some(Player::X) => return Ended(Win(X)),
                    Some(Player::O) => return Ended(Win(O)),
                    None => continue,
                }
            }
        }

        for i in 0..BOARD_SIZE {
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
                match self.board[0][i].player() {
                    Some(Player::X) => return Ended(Win(X)),
                    Some(Player::O) => return Ended(Win(O)),
                    None => continue,
                }
            }
        }

        if usize::from(self.moves) == BOARD_SIZE * BOARD_SIZE {
            return Ended(Tie);
        }

        self.current_player = self.current_player.opposite();
        Playing(self.current_player)
    }

    pub fn state(&self) -> [[Cell; BOARD_SIZE]; BOARD_SIZE] {
        self.board
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Cell, Game, GameStatus, Player};

    #[test]
    fn test_check_win_in_rows() {
        test_win_in_rows(Player::X);
        test_win_in_rows(Player::O);
    }

    fn test_win_in_rows(player: Player) {
        let mut game = Game::new();

        let player_cell = Cell::Filled(player);
        game.board = [
            [player_cell, player_cell, player_cell],
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
        ];
        game.moves = 3;

        assert_eq!(
            game.check(),
            GameStatus::Ended(crate::game::Outcome::Win(player))
        );
    }
}
