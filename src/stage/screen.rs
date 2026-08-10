use std::{ops::AddAssign, u32};

use glam::{Vec2, Vec3, vec2, vec3};
use miniquad::KeyCode::S;
use quad_rand::gen_range;

use crate::stage::{COLUMNS, ROWS};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InstanceData {
    pos: Vec2,
    fg: f32,
    bg: f32,
    character: f32,
}

impl InstanceData {
    pub fn new(i: usize) -> Self {
        Self {
            pos: vec2((i % COLUMNS) as f32, (i / COLUMNS) as f32) * 8.0,
            fg: gen_range(000, 999) as f32,
            bg: gen_range(000, 999) as f32,
            character: gen_range(0.0, 128.0)
        }
    }

    pub fn set_color_splash(&mut self, fg: Option<f32>, bg: Option<f32>) {
        if let Some(fg) = fg {
            self.fg = fg.clamp(0.0, 999.0).trunc();
        }
        if let Some(bg) = bg {
            self.bg = bg.clamp(0.0, 999.0).trunc();
        }
    }
    pub fn set_color_rgb(&mut self, fg: Option<Vec3>, bg: Option<Vec3>) {
        let n = |i: f32| -> f32 {
            i.floor().clamp(0.0, 9.0)
        };
        if let Some(c) = fg {
            self.fg = n(c.x) * 100.0 + n(c.y) * 10.0 + n(c.z);
        }
        if let Some(c) = bg {
            self.bg = n(c.x) * 100.0 + n(c.y) * 10.0 + n(c.z);
        }
    }

    pub fn set_character(&mut self, c: f32, flashing: Option<bool>, inverted: Option<bool>) {
        self.character = c + self.character.fract();
    }
    // TODO: flashing and inverted
}

pub struct Screen {
    instance_data: [InstanceData; COLUMNS*ROWS],
}

impl Screen {
    pub fn new() -> Self {
        let mut i = usize::MAX;
        // initialise the screen with garbage values (because its coooool :3)
        let instance_data = (0..COLUMNS*ROWS)
            .map(|_| {
                i = i.wrapping_add(1);
                InstanceData::new(i)
            })
            .collect::<Vec<InstanceData>>()
            .try_into()
            .expect("uh oh, failed to assemble screen. this should never happen unless ive been reaaaally silly...");
        Self { instance_data }
    }

    pub fn char_xy(&mut self, x: usize, y: usize) -> &mut InstanceData {
        &mut self.instance_data[x + y * COLUMNS]
    }

    pub fn set_character_xy(&mut self, x: usize, y: usize, fg: Option<Vec3>, bg: Option<Vec3>, ) {
        // self.instance_data[x + y * COLUMNS]
    }

    pub fn test_texture(&mut self) {
        // rrrg ggbb brrr gggb bbcc cccc cif- ----
        for i in 0..COLUMNS*ROWS {
            let x = i % COLUMNS;
            let y = i / COLUMNS;

            // let character = (((x % 16) + (y % 8) * 16) % 128) as u32;
            // let character = 128;
            
            let step = x / (COLUMNS / 6);
            let a = ((y / (ROWS / 10)) as f32).min(9.0);
            // println!("{:?}", a);
            let r = if matches!(step, 0|1|5) { a } else { 0.0 };
            let g = if matches!(step, 1|2|3) { a } else { 0.0 };
            let b = if matches!(step, 3|4|5) { a } else { 0.0 };
            let col = vec3(r, g, b);

            self.instance_data[i].set_color_rgb(Some(col), Some(9.0 - col));
            self.instance_data[i].set_character((((x % 16) + 16 - (y % 8) * 16) % 128) as f32, None, None);

            // self.instance_data[i].data = 0
            //  + ((r & 7) << 20)
            //  + ((g & 7) << 17)
            //  + ((b & 7) << 14)
            //  + (0b1 << 6) * if (x+y) % 2 == 0 { 0 } else { 1 };
            // self.instance_data[i].data = 0b111_000_000__111_111_111_1000000_0_0_00000u32.to_be();
            // self.instance_data[i].data = 0b11111111_11111101u32;
            // self.instance_data[i].data = 0b110;
            // println!("{:?}", self.instance_data[i].data);

            // self.instance_data[i].set_fg(000.0);
            // self.instance_data[i].set_bg(090.0);
            // self.instance_data[i].set_bg_color(100.0);

            // if next != prev {
            //     println!("diff: {prev} -> {next}");
            // }
            // self.instance_data[i].set_bg(r * 100.0 + g * 10.0 + b);
            // self.instance_data[i].set_fg(b * 100.0 + r * 10.0 + g);
            // self.instance_data[i].set_character((((x % 16) + 16 - (y % 8) * 16) % 128) as f32);

            // println!("{:#034b}", self.instance_data[i].data);
        }
    }

    pub fn instance_data(&self) -> &[InstanceData; COLUMNS*ROWS] {
        &self.instance_data
    }
}