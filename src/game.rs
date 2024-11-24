use crate::game::Cell::Empty;
use crate::game::Outcome::{Tie, Win};
use crate::game::Player::{O, X};
use crate::game::Status::{Ended, Playing};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
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
pub enum Status {
    Playing(Player),
    Ended(Outcome),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveError {
    AlreadyOccupied,
    OutOfBounds,
}

pub struct State {
    pub board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    pub status: Status,
}

pub const BOARD_SIZE: usize = 3;
pub struct Game {
    board: [[Cell; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
    moves: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: X,
            moves: 0,
        }
    }

    pub fn apply(&mut self, i: usize, j: usize) -> std::result::Result<Status, MoveError> {
        if i > 2 || j > 2 {
            return Err(MoveError::OutOfBounds);
        }

        if self.board[i][j] != Empty {
            return Err(MoveError::AlreadyOccupied);
        }

        self.board[i][j] = Cell::Filled(self.current_player);
        self.moves += 1;

        let status = self.check();
        let status = match status {
            Playing(player) => {
                self.current_player = player.opposite();
                Playing(self.current_player)
            },
            _ => status,
        };
        Ok(status)
    }

    fn check(&self) -> Status {
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

        Playing(self.current_player)
    }

    pub fn state(&self) -> State {
        State {
            board: self.board,
            status: self.check(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Cell, Game, Player, Status};

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
            Status::Ended(crate::game::Outcome::Win(player))
        );
    }
}
