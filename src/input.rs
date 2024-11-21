use std::io;

fn parse_input(input: &str) -> Result<(usize, usize), ()> {
    let mut parts = input.trim().split(',');

    let i: usize = match parts.next().unwrap_or_default().parse() {
        Ok(digit) => digit,
        Err(_) => return Err(()),
    };

    let j: usize = match parts.next().unwrap_or_default().parse() {
        Ok(digit) => digit,
        Err(_) => return Err(()),
    };

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
            Err(_) => println!("incorrect input"),
        }
    }
}
