use crate::game::{Cell, Player, State, BOARD_SIZE};

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
    state: State,
}

impl Display {
    pub fn new(state: State) -> Self {
        Self {
            term: ratatui::init(),
            msg: String::new(),
            state,
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

impl super::Display for Display {
    fn on_move(&mut self, status: std::result::Result<Status, MoveError>) {
        match status {
            Ok(status) => match status {
                Status::Playing(player) => {
                    self.msg = format!("Current player: {player}\n");
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

    fn draw(&mut self) {
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
            let board_content = render_board(&self.state.board);
            let game_area = Paragraph::new(board_content)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Game Board")
                        .padding(Padding::new(0, 0, chunks[1].height / 2, 0)),
                );
            f.render_widget(game_area, chunks[1]);

            // -- bottom bar --
            let bottom_bar = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[2]);

            // 1. Message bar
            let message_bar = Paragraph::new(self.msg.clone())
                .block(Block::default().borders(Borders::ALL).title("Message"));
            f.render_widget(message_bar, bottom_bar[0]);

            // 2. Instructions bar
            let instructions = "-Enter <quit> to exit the game\n-Choose a position (Format: i,j):";
            let message_bar = Paragraph::new(instructions)
                .block(Block::default().borders(Borders::ALL).title("Instructions"));
            f.render_widget(message_bar, bottom_bar[1]);
        });
    }

    fn on_input(
        &mut self,
        res: &std::result::Result<super::super::input::Result, std::num::ParseIntError>,
    ) {
        match res {
            Ok(_) => {}
            Err(e) => self.msg = format!("Incorrect input: {e}\n"),
        };
    }
    
    fn update(&mut self, state: State) {
        self.state = state;
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
