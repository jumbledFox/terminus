// Screen struct that stores mesh, duplicate array, and has functions for drawing shapes
// keeps track of dirty rows to be updated / redrawn,

use std::println;

use glam::{Mat4, vec2};
use quad_rand::gen_range;

use crate::stage::{COLUMNS, Ctx, ROWS, U_COLUMNS, U_ROWS, screen::mesh::ScreenMesh};

mod mesh;

#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Cell {
    ch: u8,
    fg: u8,
    bg: u8,
    attrib: u8
}

pub struct Screen {
    mesh: ScreenMesh,
    cells: [Cell; U_COLUMNS * U_ROWS]
}

impl Screen {
    pub fn new(ctx: &mut Ctx) -> Self {
        let mut cells = [Cell { ch: 0, fg: 0, bg: 0, attrib: 0}; U_COLUMNS*U_ROWS];
        for (i, c) in cells.iter_mut().enumerate() {
            c.ch = gen_range(0, 127);
            c.fg = gen_range(0, 255);
            c.bg = gen_range(0, 255);
            println!("{:?}", (i, &c));
        }

        let mut screen = Self {
            mesh: ScreenMesh::new(ctx),
            cells,
        };
        // for i in 0..=26 {
        //     screen.cells[i] = Cell { ch: 64 + i as u8, fg: 2, bg: 3, attrib: 0 }
        // }
        screen.update_mesh(ctx);
        screen
    }

    pub fn update_mesh(&mut self, ctx: &mut Ctx) {
        self.mesh.update_texture(ctx, bytemuck::cast_slice(&self.cells[..]));
        // self.mesh.update_texture(ctx, &[255; 4*U_COLUMNS*U_ROWS]);
    }

    pub fn draw(&mut self, ctx: &mut Ctx) {
        self.update_mesh(ctx);
        self.mesh.draw(ctx);
    }
}