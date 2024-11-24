pub mod term;
pub mod tui;

pub enum Result {
    Position(usize, usize),
    Exit,
    None,
}

pub trait Input {
    fn get(&mut self) -> std::result::Result<Result, std::num::ParseIntError>;
}

pub fn parse_input(input: &str) -> std::result::Result<Result, std::num::ParseIntError> {
    let mut parts = input.trim().split(',');

    let i: usize = parts.next().unwrap_or("").parse()?;
    let j: usize = parts.next().unwrap_or("").parse()?;

    Ok(Result::Position(i, j))
}