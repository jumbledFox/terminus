use std::println;

use glam::{Vec2, Vec3, vec2};
use miniquad::{EventHandler, RenderingBackend, window::{self, screen_size}};

use crate::stage::{brains::Brains, input::Input, screen::Screen};

pub mod screen;
pub mod input;
pub mod brains;

pub const COLUMNS: u8 = 80+40;
pub const ROWS:    u8 = 48+24;
pub const U_COLUMNS: usize = COLUMNS as usize;
pub const U_ROWS:    usize = ROWS    as usize;

pub type Ctx = Box<dyn RenderingBackend>;

pub struct Stage {
    ctx: Box<dyn RenderingBackend>,
    screen: Screen,
    input: Input,
    brains: Brains,
}

impl Stage {
    pub fn new() -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();
        let screen = Screen::new(&mut ctx);
        let input = Input::new();
        Self {
            ctx,
            screen,
            input,
            brains: Brains::new(),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.brains.update(&mut self.screen, &self.input);
        // for i in 0..26 {
        //     self.screen.char_xy(i+4, 12).set_character(64.0 + i as f32, None, None);
        //     self.screen.char_xy(i+4, 12).set_color_splash(Some(999.0), Some(900.0));
        // }
    }

    fn char_event(&mut self, _character: char, _keymods: miniquad::KeyMods, _repeat: bool) {
        println!("{:?}", _character);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let (sw, sh) = screen_size();
        self.input.mouse_move(&self.screen, vec2(x, y) / vec2(sw, sh));
    }

    fn draw(&mut self) {
        // Make sure a Vec2 is represented as #[repr(C)](f32, f32)
        assert_eq!(std::mem::size_of::<Vec2>(), 8);
        // Make sure a Vec3 is represented as #[repr(C)](f32, f32, f32)
        assert_eq!(std::mem::size_of::<Vec3>(), 12);

        self.screen.draw(&mut self.ctx);
    }
}