use glam::{ISizeVec2, Vec2};

use crate::stage::screen::Screen;

pub struct Input {
    mouse_pixel: Option<ISizeVec2>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_pixel: None,
        }
    }

    pub fn mouse_move(&mut self, screen: &Screen, local: Vec2) {
        println!("{:?}", local)
    }
}