use crate::game::{Cell, Player, BOARD_SIZE};

use super::super::{game::MoveError, game::Outcome, Status};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::{
    style::{Color, Style},
    text::Span,
    text::{Line, Text},
    widgets::Padding,
    widgets::{Block, Borders, Paragraph},
};

pub struct Display {
    term: ratatui::DefaultTerminal,
    msg: String,
}

impl Display {
    pub fn new() -> Self {
        Self {
            term: ratatui::init(),
            msg: String::new(),
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

impl super::Display for Display {
    fn on_change(&mut self, status: std::result::Result<Status, MoveError>) {
        match status {
            Ok(status) => match status {
                Status::Playing(player) => {
                    self.msg =
                        format!("Current player: {player}\nChoose a position (Format: i,j):");
                }
                Status::Ended(Outcome::Tie) => {
                    self.msg = format!("Game result: Tie");
                }
                Status::Ended(Outcome::Win(player)) => {
                    self.msg = format!("Game result: {player} wins");
                }
            },
            Err(e) => match e {
                MoveError::AlreadyOccupied => {
                    self.msg = format!("Incorrect move: cell already occupied")
                }
                MoveError::OutOfBounds => {
                    self.msg = format!("Incorrect move: input is out of bounds")
                }
            },
        }
    }

    fn draw(&mut self, state: [[crate::game::Cell; 3]; 3]) {
        _ = self.term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(15),
                    Constraint::Percentage(70),
                    Constraint::Percentage(15),
                ])
                .split(f.area());

            // Title bar
            let title = Paragraph::new(Span::styled(
                "Tic-Tac-Toe",
                Style::default().fg(Color::Yellow),
            ))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Game area
            let board_content = render_board(&state);
            let game_area = Paragraph::new(board_content)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Game Board")
                        .padding(Padding::new(0, 0, chunks[1].height / 2, 0)),
                );
            f.render_widget(game_area, chunks[1]);

            // Message bar
            let message_bar = Paragraph::new(self.msg.clone())
                .block(Block::default().borders(Borders::ALL).title("Message"));
            f.render_widget(message_bar, chunks[2]);
        });
    }

    fn on_input(
        &mut self,
        res: &std::result::Result<super::super::input::Result, std::num::ParseIntError>,
    ) {
        match res {
            Ok(_) => {}
            Err(e) => self.msg = format!("Incorrect input{e}\nChoose a position (Format: i,j):"),
        };
    }
}

fn render_board(board: &[[Cell; BOARD_SIZE]; BOARD_SIZE]) -> Text<'static> {
    let mut lines = Vec::new();

    for row in board {
        let spans: Vec<Span> = row
            .iter()
            .map(|cell| match cell {
                Cell::Empty => Span::raw("[   ]"),
                Cell::Filled(Player::X) => Span::styled("[ X ]", Style::default().fg(Color::Blue)),
                Cell::Filled(Player::O) => Span::styled("[ O ]", Style::default().fg(Color::Red)),
            })
            .collect();
        lines.push(Line::from(spans));
    }

    Text::from(lines)
}
