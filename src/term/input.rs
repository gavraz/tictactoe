use super::super::input;

pub struct Input {
    buff: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            buff: String::new(),
        }
    }
}

impl input::Input for Input {
    fn get(&mut self) -> std::result::Result<input::Result, std::num::ParseIntError> {
        self.buff.clear();

        std::io::stdin()
            .read_line(&mut self.buff)
            .expect("failed to read from stdin");

        if self.buff.trim_end() == "quit" {
            return Ok(input::Result::Exit);
        }

        input::parse_input(&self.buff)
    }
    
    fn wait_exit(&mut self) {}
}