use std::io;

pub enum Result {
    Position(usize, usize),
    Exit,
    None,
}

pub trait Input {
    fn get(&mut self) -> std::result::Result<Result, std::num::ParseIntError>;
}

fn parse_input(input: &str) -> std::result::Result<Result, std::num::ParseIntError> {
    let mut parts = input.trim().split(',');

    let i: usize = parts.next().unwrap_or("").parse()?;
    let j: usize = parts.next().unwrap_or("").parse()?;

    Ok(Result::Position(i, j))
}

pub struct TerminalInput {
    buff: String,
}

impl TerminalInput {
    pub fn new() -> Self {
        Self {
            buff: String::new(),
        }
    }
}

impl Input for TerminalInput {
    fn get(&mut self) -> std::result::Result<Result, std::num::ParseIntError> {
        self.buff.clear();

        io::stdin()
            .read_line(&mut self.buff)
            .expect("failed to read from stdin");

        if self.buff.trim_end() == "quit" {
            return Ok(Result::Exit);
        }

        parse_input(&self.buff)
    }
}

use crossterm::event::{self, Event, KeyCode};

pub struct CrossTermInput {
    buff: String,
}

impl CrossTermInput {
    pub fn new() -> Self {
        Self{ buff: String::new() }
    }
}

impl Input for CrossTermInput {
    fn get(&mut self) -> std::result::Result<Result, std::num::ParseIntError> {
        if event::poll(std::time::Duration::from_millis(250)).expect("not good") {
            if let Event::Key(key_event) = event::read().expect("not good") {
                match key_event.code {
                    KeyCode::Char(c) => {
                        self.buff.push(c);
                        // print!("{}", c);
                        // io::stdout().flush()?; // Flush to show the typed character
                    }
                    KeyCode::Enter => {
                        // println!(); // Newline after Enter key
                        let input = std::mem::take(&mut self.buff);
                        if input == "quit" {
                            return Ok(Result::Exit);
                        } 
                        return parse_input(&input);
                    }
                    KeyCode::Backspace => {
                        if !self.buff.is_empty() {
                            self.buff.pop(); // Remove last character from buffer
                            // print!("\r{}\r{}", " ".repeat(self.buff.len() + 1), input); // Overwrite line
                            // io::stdout().flush()?; // Flush updated input
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Result::None)
    }
}