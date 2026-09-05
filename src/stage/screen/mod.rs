// Screen struct that stores mesh, duplicate array, and has functions for drawing shapes
// keeps track of dirty rows to be updated / redrawn,

use glam::{Mat4, vec2};

use crate::stage::{COLUMNS, Ctx, ROWS, U_COLUMNS, U_ROWS, screen::mesh::ScreenMesh};

mod mesh;

#[derive(Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
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
        let mut screen = Self {
            mesh: ScreenMesh::new(ctx),
            cells: [Cell {
                ch: 64, fg: 255, bg: 255, attrib: 0
            }; U_COLUMNS * U_ROWS]
        };
        screen.update_mesh(ctx);
        screen
    }

    pub fn update_mesh(&mut self, ctx: &mut Ctx) {
        self.mesh.update_texture(ctx, bytemuck::cast_slice(&self.cells[..]));
    }

    pub fn draw(&self, ctx: &mut Ctx) {
        self.mesh.draw(ctx);
    }
}