use std::{ops::AddAssign, u32};

use glam::{Vec2, vec2};
use quad_rand::gen_range;

use crate::stage::{COLUMNS, ROWS};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InstanceData {
    data: f32,
    pos: Vec2,
}

pub struct Screen {
    instance_data: [InstanceData; COLUMNS*ROWS],
}

impl Screen {
    pub fn new() -> Self {
        let mut i = usize::MAX;
        let instance_data = (0..COLUMNS*ROWS)
            .map(|_| {
                i = i.wrapping_add(1);
                InstanceData {
                    pos: vec2((i % COLUMNS) as f32, (i / COLUMNS) as f32) * 8.0,
                    data: gen_range(0.0, f32::MAX),
                }
            })
            .collect::<Vec<InstanceData>>()
            .try_into()
            .expect("uh oh, failed to assemble screen. this should never happen unless ive been reaaaally silly...");
        let mut s = Self { instance_data };
        s.test_texture();
        s
    }

    pub fn test_texture(&mut self) {
        // rrrg ggbb brrr gggb bbcc cccc cif- ----
        for i in 0..COLUMNS*ROWS {
            let x = i % COLUMNS;
            let y = i / COLUMNS;

            // let character = (((x % 16) + (y % 8) * 16) % 128) as u32;
            // let character = 128;
            
            let step = x / (COLUMNS / 6);
            println!("{:?}", step);
            let a = (y / (ROWS / 8)) as u32;
            let a = 7;
            let r = if matches!(step, 0) { a } else { 0 };
            let g = if matches!(step, 1) { a } else { 0 };
            let b = if matches!(step, 2) { a } else { 0 };

            // self.instance_data[i].data = 0
            //  + ((r & 7) << 20)
            //  + ((g & 7) << 17)
            //  + ((b & 7) << 14)
            //  + (0b1 << 6) * if (x+y) % 2 == 0 { 0 } else { 1 };
            // self.instance_data[i].data = 0b111_000_000__111_111_111_1000000_0_0_00000u32.to_be();
            // self.instance_data[i].data = 0b11111111_11111101u32;
            self.instance_data[i].data = 2.0;
            println!("{:?}", self.instance_data[i].data);
            // println!("{:#034b}", self.instance_data[i].data);
        }

        println!("InstanceData size = {}", std::mem::size_of::<InstanceData>());
        println!("InstanceData align = {}", std::mem::align_of::<InstanceData>());
        println!("Vec2 size = {}", std::mem::size_of::<Vec2>());
    }

    pub fn instance_data(&self) -> &[InstanceData; COLUMNS*ROWS] {
        &self.instance_data
    }
}