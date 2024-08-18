use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use super::Point;

pub struct Renderer {
    pub canvas: WindowCanvas,
    pixel_size: u32,
}

impl Renderer {
    pub fn new( window: Window, pixel_size: u32 ) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer {
            canvas,
            pixel_size
        })
    }

    pub fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point{x, y} = point;
        self.canvas.fill_rect(Rect::new(
            x * self.pixel_size as i32,
            y * self.pixel_size as i32,
            self.pixel_size,
            self.pixel_size,
        ))?;

        Ok(())
    }
    
    pub fn draw(&mut self, pixels: &Vec<Vec<bool>>) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        for (x, row) in pixels.iter().enumerate() {
            for (y, px) in row.iter().enumerate() {
                if *px {
                    self.draw_dot(&Point{ x: (x as i32), y: (y as i32)})?;
                }
                
            }
        }
        self.canvas.present();

        Ok(())
    }
}

