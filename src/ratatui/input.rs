use crossterm::event::{self, Event, KeyCode};
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
        if event::poll(std::time::Duration::from_millis(250)).expect("not good") {
            if let Event::Key(key_event) = event::read().expect("not good") {
                match key_event.code {
                    KeyCode::Char(c) => {
                        self.buff.push(c);
                    }
                    KeyCode::Enter => {
                        let input = std::mem::take(&mut self.buff);
                        if input == "quit" {
                            return Ok(input::Result::Exit);
                        }
                        return input::parse_input(&input);
                    }
                    KeyCode::Backspace => {
                        if !self.buff.is_empty() {
                            self.buff.pop();
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(input::Result::None)
    }

    fn wait_exit(&mut self) {
        loop {
            match self.get() {
                Ok(input::Result::Exit) => return,
                _ => (),
            };
        }
    }
}
