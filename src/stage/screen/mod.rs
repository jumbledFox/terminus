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
        // fill the screen with garbage :3
        for c in cells.iter_mut() {
            c.ch = gen_range(0, 127);
            c.fg = gen_range(0, 255);
            c.bg = gen_range(0, 255);
        }

        for i in 0..26 {
            cells[i] = Cell { ch: 64 + i as u8, fg: 4, bg: 7*16, attrib: 0 }
        }

        for x in 0..20 {
            for y in 1..10 {
                cells[x+y*U_COLUMNS] = Cell { ch: 0, fg: 0, bg: 255, attrib: 0 };
            }
        }

        cells[1+2*U_COLUMNS] = Cell { ch: 36, fg: 80, bg: 255, attrib: 0 };
        cells[1+3*U_COLUMNS] = Cell { ch: 41, fg: 80, bg: 255, attrib: 0 };
        cells[2+3*U_COLUMNS] = Cell { ch: 34, fg: 80, bg: 255, attrib: 0 };

        cells[3+2*U_COLUMNS] = Cell { ch: 3,  fg: 255, bg: 88, attrib: 0 };
        cells[3+3*U_COLUMNS] = Cell { ch: 33, fg: 138, bg: 88, attrib: 0 };
        cells[3+4*U_COLUMNS] = Cell { ch: 34, fg: 138, bg: 255, attrib: 0 };

        cells[4+2*U_COLUMNS] = Cell { ch: 34, fg: 255, bg: 88, attrib: 0 };
        cells[4+3*U_COLUMNS] = Cell { ch: 0,  fg: 0,   bg: 88, attrib: 0 };
        cells[4+4*U_COLUMNS] = Cell { ch: 43, fg: 255, bg: 88, attrib: 0 };

        cells[5+2*U_COLUMNS] = Cell { ch: 95, fg: 255, bg: 88, attrib: 0 };
        cells[5+3*U_COLUMNS] = Cell { ch: 95, fg: 255, bg: 88, attrib: 0 };
        cells[5+4*U_COLUMNS] = Cell { ch: 21, fg: 88, bg: 138, attrib: 0 };

        for i in 0..7 {
            cells[6+i+2*U_COLUMNS] = Cell { ch: 3, fg: 255, bg: 88, attrib: 0 };
            cells[6+i+3*U_COLUMNS] = Cell { ch: 0, fg: 0, bg: 88, attrib: 0 };
            cells[6+i+4*U_COLUMNS] = Cell { ch: 3, fg: 138, bg: 255, attrib: 0 };
        }


        let mut screen = Self {
            mesh: ScreenMesh::new(ctx),
            cells,
        };

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