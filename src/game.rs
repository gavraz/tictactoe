use crate::game::Cell::{Empty, PlayerO, PlayerX};
use crate::game::Player::{X, O};
use crate::game::GameResult::{AlreadyTaken, InvalidMove, Playing, Tie, Winner};

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    PlayerX,
    PlayerO,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn to_cell(&self) -> Cell {
        match self {
            X => { PlayerX }
            O => { PlayerO }
        }
    }

    fn opposite(&self) -> Player {
        match self {
            X => { O }
            O => { X }
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameResult {
    Playing(Player),
    Tie,
    Winner(Player),
    AlreadyTaken,
    InvalidMove,
}

pub struct Game {
    board: [[Cell; 3]; 3],
    current_player: Player,
    moves: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[Empty; 3]; 3],
            current_player: X,
            moves: 0,
        }
    }

    pub fn apply(&mut self, i: usize, j: usize) -> GameResult {
        if i > 2 || j > 2 {
            return InvalidMove;
        }

        if self.board[i][j] != Empty {
            return AlreadyTaken;
        }

        self.board[i][j] = self.current_player.to_cell();
        self.moves += 1;

        return self.check();
    }

    fn check(&mut self) -> GameResult {
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            match self.board[0][0] {
                PlayerX => return Winner(X),
                PlayerO => return Winner(O),
                Empty => (),
            }
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            match self.board[0][2] {
                PlayerX => return Winner(X),
                PlayerO => return Winner(O),
                Empty => (),
            }
        }

        for row in self.board.iter() {
            if row[0] == row[1] && row[1] == row[2] {
                return match row[0] {
                    PlayerX => { Winner(X) }
                    PlayerO => { Winner(O) }
                    Empty => { continue; }
                };
            }
        }

        for i in 0..3 {
            if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
                return match self.board[0][i] {
                    PlayerX => { Winner(X) }
                    PlayerO => { Winner(O) }
                    Empty => { continue; }
                };
            }
        }

        if self.moves == 9 {
            return Tie;
        }

        self.current_player = self.current_player.opposite();
        Playing(self.current_player)
    }

    pub fn state(&self) -> [[Cell; 3]; 3] {
        return self.board.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{Cell, Game, GameResult, Player};

    #[test]
    fn test_check_win_in_rows() {
        test_win_in_rows(Player::X);
        test_win_in_rows(Player::O);
    }

    fn test_win_in_rows(player: Player) {
        let mut game = Game::new();

        let player_cell = player.to_cell();
        game.board = [
            [player_cell, player_cell, player_cell],
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
        ];
        game.moves = 3;

        assert_eq!(game.check(), GameResult::Winner(player));
    }
}