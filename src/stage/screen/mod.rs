// Screen struct that stores mesh, duplicate array, and has functions for drawing shapes
// keeps track of dirty rows to be updated / redrawn,

use quad_rand::gen_range;

use crate::stage::{COLUMNS, Ctx, U_COLUMNS, U_ROWS, screen::mesh::ScreenMesh};

mod mesh;

pub enum ViewMode {
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

        let mut screen = Self {
            mesh: ScreenMesh::new(ctx),
            cells,
        };

        screen.update_mesh(ctx);
        screen
    }

    pub fn xy_to_index(x: usize, y: usize) -> usize {
        x + y * U_COLUMNS
    }

    pub fn draw_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[Self::xy_to_index(x, y)] = cell;
    }

    pub fn draw_line(&mut self) {
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    }

    pub fn fill(&mut self, cell: Cell) {
        self.cells = [cell; U_COLUMNS*U_ROWS];
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