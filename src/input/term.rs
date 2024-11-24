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

impl super::Input for TerminalInput {
    fn get(&mut self) -> std::result::Result<super::Result, std::num::ParseIntError> {
        self.buff.clear();

        std::io::stdin()
            .read_line(&mut self.buff)
            .expect("failed to read from stdin");

        if self.buff.trim_end() == "quit" {
            return Ok(super::Result::Exit);
        }

        super::parse_input(&self.buff)
    }
}