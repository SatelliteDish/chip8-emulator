use super::Chip8Error;
use std::error::Error;

pub struct Cpu {
    stack: Stack
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            stack: Stack::new()
        }
    }
}

struct Stack {
    stack: Vec<u8>
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::new()
        }
    }

    pub fn push(&mut self, item: u8) -> Result <(), Box<dyn Error>> {
        if self.stack.len() >= 64 {
            Err(Box::new(Chip8Error::new("Stack Overflow".to_string())))
        } else {
            self.stack.push(item);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.remove(0))
        }
    }

    pub fn peek(&self) -> Option<&u8> {
        if self.stack.is_empty() {
            None
        } else {
            Some(&self.stack[0])
        }
    }
}
