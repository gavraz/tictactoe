use std::io;

pub enum Result {
    Position(usize, usize),
    Exit
}

pub trait Input {
    fn get(&self) -> Result;
}

fn parse_input(input: &str) -> std::result::Result<(usize, usize), std::num::ParseIntError> {
    let mut parts = input.trim().split(',');

    let i: usize = parts.next().unwrap().parse()?;
    let j: usize = parts.next().unwrap().parse()?;

    Ok((i, j))
}

pub struct TerminalInput {}

impl Input for TerminalInput {
    fn get(&self) -> Result {
        let mut input = String::new();
        loop {
            input.clear();

            println!("Choose a position (Format: i,j):");
            io::stdin().read_line(&mut input).expect("failed to read from stdin");

            if input.trim_end() == "quit" {
                return Result::Exit;
            }

            match parse_input(&input) {
                Ok((i, j)) => break Result::Position(i,j),
                Err(e) => println!("incorrect input: {}", e),
            }
        }
    }
}
