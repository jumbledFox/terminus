// Screen struct that stores mesh, duplicate array, and has functions for drawing shapes
// keeps track of dirty rows to be updated / redrawn,

use std::println;

use glam::{Mat4, vec2};
use quad_rand::gen_range;

use crate::stage::{COLUMNS, Ctx, ROWS, U_COLUMNS, U_ROWS, screen::mesh::ScreenMesh};

mod mesh;

enum ViewMode {
    Stretch,
    Fit,
    PixelPerfect,
}

const A_INVERT: u8 = 0b0000_0001;
const A_FLASH:  u8 = 0b0000_0010;

#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Cell {
    ch: u8,
    fg: u8,
    bg: u8,
    attrib: u8
}

impl Cell {
    pub fn new(ch: u8, fg: u8, bg: u8, attrib: u8) -> Self {
        Self { ch, fg, bg, attrib }
    }
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

        for y in 0..U_ROWS {
            for x in 0..U_COLUMNS {
                let r = ((x * 8 * 2) / U_COLUMNS) % 8;
                let g = ((y * 8 * 2) / U_ROWS)    % 8;
                let b = if x >= U_COLUMNS/2 { 1 } else { 0 } + if y >= U_ROWS/2 { 2 } else { 0 };

                let fg = ((r << 5) + (g << 2) + b) as u8;

                let ch = match gen_range(0, 5) {
                    0 => 63,
                    1 => 95,
                    2 => 111,
                    _ => 0
                };

                // let ch = match gen_range(0, 4) {
                //     0 => 63,
                //     1 => 46,
                //     2 => 62,
                //     _ => 0
                // };

                cells[x+y*U_COLUMNS] = Cell::new(ch, fg, 0, 0);
            }
        }

        for s in 0..4 {
            
        }


        for i in 0..26 {
            cells[i] = Cell { ch: 64 + i as u8, fg: 225, bg: 255, attrib: 0 }
        }

        // for x in 0..20 {
        //     for y in 1..10 {
        //         cells[x+y*U_COLUMNS] = Cell::new(0, 0, 0, 0);
        //     }
        // }

        cells[1+2*U_COLUMNS] = Cell { ch: 36, fg: 229, bg: 0, attrib: 0 };
        cells[1+3*U_COLUMNS] = Cell { ch: 41, fg: 229, bg: 0, attrib: 0 };
        cells[2+3*U_COLUMNS] = Cell { ch: 34, fg: 229, bg: 0, attrib: 0 };

        cells[3+2*U_COLUMNS] = Cell { ch: 3,  fg: 0, bg: 61, attrib: 0 };
        cells[3+3*U_COLUMNS] = Cell { ch: 33, fg: 49, bg: 61, attrib: 0 };
        cells[3+4*U_COLUMNS] = Cell { ch: 34, fg: 49, bg: 0, attrib: 0 };

        cells[4+2*U_COLUMNS] = Cell { ch: 34, fg: 0, bg: 61, attrib: 0 };
        cells[4+3*U_COLUMNS] = Cell { ch: 0,  fg: 0,   bg: 61, attrib: 0 };
        cells[4+4*U_COLUMNS] = Cell { ch: 43, fg: 0, bg: 61, attrib: 0 };

        cells[5+2*U_COLUMNS] = Cell { ch: 95, fg: 0, bg: 61, attrib: 0 };
        cells[5+3*U_COLUMNS] = Cell { ch: 95, fg: 0, bg: 61, attrib: 0 };
        cells[5+4*U_COLUMNS] = Cell { ch: 21, fg: 61, bg: 49, attrib: 0 };

        for i in 0..7 {
            cells[6+i+2*U_COLUMNS] = Cell { ch: 3, fg: 0, bg: 61, attrib: 0 };
            cells[6+i+3*U_COLUMNS] = Cell { ch: 0, fg: 0, bg: 61, attrib: 0 };
            cells[6+i+4*U_COLUMNS] = Cell { ch: 3, fg: 49, bg: 0, attrib: 0 };
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
        // self.mesh.update_texture(ctx, &[0b111_000_11; 4*U_COLUMNS*U_ROWS]);
    }

    pub fn draw(&mut self, ctx: &mut Ctx) {
        self.update_mesh(ctx);
        self.mesh.draw(ctx);
    }
}