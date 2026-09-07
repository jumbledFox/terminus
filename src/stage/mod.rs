use std::{f32::consts::PI, io::Cursor, println};

use glam::{Mat4, Vec2, Vec3, vec2, vec3};
use image::ImageReader;
use miniquad::{Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, FilterMode, KeyCode::T, MipmapFilterMode, Pipeline, PipelineParams, RenderingBackend, ShaderMeta, ShaderSource, TextureId, UniformBlockLayout, UniformDesc, UniformType, UniformsSource, VertexAttribute, VertexFormat, VertexStep, window::{self, screen_size}};
use quad_rand::gen_range;

use crate::stage::{input::Input, screen::Screen};

pub mod input;
pub mod screen;

pub const COLUMNS: u8 = 80+40;
pub const ROWS:    u8 = 48+24;
pub const U_COLUMNS: usize = COLUMNS as usize;
pub const U_ROWS:    usize = ROWS    as usize;

pub type Ctx = Box<dyn RenderingBackend>;

pub struct Stage {
    ctx: Box<dyn RenderingBackend>,
    screen: Screen,
    input: Input,
    // program
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
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
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