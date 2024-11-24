use crossterm::event::{self, Event, KeyCode};

pub struct Input {
    buff: String,
}

impl Input {
    pub fn new() -> Self {
        Self{ buff: String::new() }
    }
}

impl super::Input for Input {
    fn get(&mut self) -> std::result::Result<super::Result, std::num::ParseIntError> {
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
                            return Ok(super::Result::Exit);
                        } 
                        return super::parse_input(&input);
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

        Ok(super::Result::None)
    }
    
    fn wait_exit(&mut self) {
        loop {
            match self.get() {
                Ok(super::Result::Exit) => return,
                _ => (),
            };
        }
    }
}