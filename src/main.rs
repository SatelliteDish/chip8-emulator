
mod chip8;
use chip8::Chip8;




pub fn main() {
    let mut chip8 = Chip8::new();
    chip8.start();
}
