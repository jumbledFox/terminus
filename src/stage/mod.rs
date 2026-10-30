use std::println;

use glam::{Vec2, Vec3, vec2};
use miniquad::{
    EventHandler, RenderingBackend,
    window::{self, screen_size},
};

use crate::stage::{
    brains::Brains,
    input::Input,
    screen::{Scale, Screen},
};

pub mod brains;
pub mod input;
pub mod programs;
pub mod screen;

pub const COLUMNS: u8 = 120;
pub const ROWS: u8 = 72;
pub const U_COLUMNS: usize = COLUMNS as usize;
pub const U_ROWS: usize = ROWS as usize;

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
        self.input.end();
    }

    fn key_down_event(
        &mut self,
        keycode: miniquad::KeyCode,
        _keymods: miniquad::KeyMods,
        repeat: bool,
    ) {
        self.input.key_down_event(keycode, repeat);
    }
    fn key_up_event(&mut self, keycode: miniquad::KeyCode, _keymods: miniquad::KeyMods) {
        self.input.key_up_event(keycode);
    }

    fn mouse_enter_event(&mut self, _button: miniquad::MouseButton, _x: f32, _y: f32) {}
    fn mouse_leave_event(&mut self) {}
    fn window_minimized_event(&mut self) {}
    fn window_restored_event(&mut self) {}

    fn char_event(&mut self, _character: char, _keymods: miniquad::KeyMods, _repeat: bool) {
        // TODO
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        let (sw, sh) = screen_size();
        self.input
            .mouse_move_event(&self.screen, vec2(x, y) / vec2(sw, sh));
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.screen.resize(Some((width, height)));
    }

    fn draw(&mut self) {
        // Make sure a Vec2 is represented as #[repr(C)](f32, f32)
        assert_eq!(std::mem::size_of::<Vec2>(), 8);
        // Make sure a Vec3 is represented as #[repr(C)](f32, f32, f32)
        assert_eq!(std::mem::size_of::<Vec3>(), 12);

        self.screen.draw(&mut self.ctx);
    }
}
