use miniquad::EventHandler;

pub mod input;
pub mod screen;

pub const COLUMNS: usize = 42;
pub const ROWS:    usize = 80;

const CHARS_TEXTURE: &[u8] = include_bytes!("../../res/chars.png");
const CHARS_VERT: &str = include_str!("chars.vert");
const CHARS_FRAG: &str = include_str!("chars.frag");

pub struct Stage {

}

impl Stage {
    pub fn new() -> Self {
        Self {  }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        
    }
}