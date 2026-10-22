use std::collections::HashSet;

use glam::{USizeVec2, Vec2, usizevec2, vec2};
use miniquad::KeyCode;

use crate::stage::{COLUMNS, ROWS, screen::Screen};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MousePos {
    pixel:   USizeVec2,
    cell:    USizeVec2,
    subcell: USizeVec2,
}

impl MousePos {
    pub fn pixel(&self) -> USizeVec2 {
        self.pixel
    }
    pub fn cell(&self) -> USizeVec2 {
        self.cell
    }
    pub fn subcell(&self) -> USizeVec2 {
        self.subcell
    }
}

pub struct Input {
    mouse_pos: Option<MousePos>,
    keys_pressed:  HashSet<KeyCode>,
    keys_down:     HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
    // TODO: Specialised code for typing 
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_pos: None,
            keys_pressed:  HashSet::with_capacity(8),
            keys_down:     HashSet::with_capacity(8),
            keys_released: HashSet::with_capacity(8),
        }
    }

    pub fn mouse_pos(&self) -> Option<MousePos> {
        self.mouse_pos
    }

    pub fn key_pressed(&self, keycode: KeyCode) -> bool {
        self.keys_pressed.contains(&keycode)
    }
    pub fn key_down(&self, keycode: KeyCode) -> bool {
        self.keys_down.contains(&keycode)
    }
    pub fn key_released(&self, keycode: KeyCode) -> bool {
        self.keys_released.contains(&keycode)
    }

    pub fn end(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
    }

    pub fn key_down_event(&mut self, keycode: KeyCode, repeat: bool) {
        if !repeat {
            self.keys_pressed.insert(keycode);
            self.keys_down.insert(keycode);
        }
    }
    pub fn key_up_event(&mut self, keycode: KeyCode) {
        self.keys_down.remove(&keycode);
        self.keys_released.insert(keycode);
    }

    pub fn mouse_move_event(&mut self, screen: &Screen, pos_norm: Vec2) {
        let (l, r, b, t) = screen.lrbt();
        let (width, height) = (
            r - l,
            t - b,
        );
        let pos_norm = pos_norm * vec2(1.0, -1.0) + vec2(0.0, 1.0);
        let local = (pos_norm * vec2(width, height) + vec2(l, b))
            .floor();
        let inside =
            local.x >= 0.0 &&
            local.y >= 0.0 &&
            local.x < COLUMNS as f32 * 8.0 &&
            local.y < ROWS    as f32 * 8.0;
        
        if !inside {
            self.mouse_pos = None;
            return;
        }

        // TODO: 
        // hmm.. if the scale changes between mouse moves, the value will be invalid
        // this doesn't rlly matter though..
        let local = local / screen.scale().f32();

        let pixel = usizevec2(
            local.x as usize,
            local.y as usize
        );

        self.mouse_pos = Some(MousePos {
            pixel,
            cell:    pixel / 8,
            subcell: pixel % 8,
        });
    }
}