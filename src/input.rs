use std::io;

pub enum Result {
    Position(usize, usize),
    Exit,
}

pub trait Input {
    fn get(&mut self) -> std::result::Result<Result, std::num::ParseIntError>;
}

fn parse_input(input: &str) -> std::result::Result<Result, std::num::ParseIntError> {
    let mut parts = input.trim().split(',');

    let i: usize = parts.next().unwrap().parse()?;
    let j: usize = parts.next().unwrap().parse()?;

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
