use std::io;

fn parse_input(input: &str) -> Result<(usize, usize), std::num::ParseIntError> {
    let mut parts = input.trim().split(',');

    let i: usize = parts.next().unwrap().parse()?;
    let j: usize = parts.next().unwrap().parse()?;

    Ok((i, j))
}

pub fn input() -> (usize, usize) {
    let mut input = String::new();

    loop {
        input.clear();

        println!("Choose a position (Format: i,j):");
        io::stdin().read_line(&mut input).expect("failed to read from stdin");

        match parse_input(&input) {
            Ok(result) => break result,
            Err(e) => println!("incorrect input: {}", e),
        }
    }
}