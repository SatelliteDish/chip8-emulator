mod renderer;
use renderer::Renderer;
use std::error::Error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::Sdl;
use std::fmt::Display;

#[derive(Debug)]
pub struct Chip8Error {
    message: String
}

impl Display for Chip8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl Error for Chip8Error {}

impl Chip8Error {
    pub fn new(message: String) -> Self {
        Chip8Error { message }
    }
}

pub struct Point { x: i32, y: i32 }

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

pub struct Chip8 {
    screen: Vec<Vec<bool>>,
    context: Sdl,
    renderer: Renderer,
    stack: Stack,
    pointer: usize,
    heap: Vec<u8>,
}

impl Chip8 {
    pub fn start(&mut self) {
        let mut event_pump = self.context.event_pump().unwrap();
        'game_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'game_loop
                    },
                    _ => {}
                }
            }
            self.screen[16][16] = true;
            let _ = self.renderer.draw(&self.screen);
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn new() -> Self {
        let pixel_size = 24;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Chip8 Emulator", 64 * pixel_size, 32 * pixel_size)
            .position_centered()
            .build()
            .unwrap();
        Chip8 {
            screen: vec![vec![false; 64]; 32],
            renderer: Renderer::new( window, pixel_size ).unwrap(),
            context: sdl_context,
            stack: Stack::new(),
            pointer: 0,
            heap: vec![0;4096]
        }
    }

    fn fetch(&mut self) -> (u8, u8) {
        let res = (self.heap[self.pointer], self.heap[self.pointer + 1]);
        self.pointer += 2;
        res
    }

    fn decode(&self) {
        todo!()
    }

    fn execute(&self) {
        todo!()
    }
}
