mod renderer;
use renderer::Renderer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::Sdl;

pub struct Point { x: i32, y: i32 }

pub struct Chip8 {
    screen: Vec<Vec<bool>>,
    context: Sdl,
    renderer: Renderer,
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
            }
        }
}
