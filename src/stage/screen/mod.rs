// Screen struct that stores mesh, duplicate array, and has functions for drawing shapes
// keeps track of dirty rows to be updated / redrawn,

use quad_rand::gen_range;

use crate::stage::{COLUMNS, Ctx, U_COLUMNS, U_ROWS, screen::{cell::Cell, mesh::ScreenMesh}};

pub mod cell;
mod mesh;

pub enum ViewMode {
    Stretch,
    Fit,
    PixelPerfect,
}

pub struct Screen {
    mesh: ScreenMesh,
    cells: [Cell; U_COLUMNS * U_ROWS],
}

impl Screen {
    pub fn new(ctx: &mut Ctx) -> Self {
        // fill the screen with garbage :3
        let cells = [Cell::random(); U_COLUMNS*U_ROWS];

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

    // TODO: Maybe not?
    pub fn draw_line(&mut self) {
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    }

    pub fn fill(&mut self, cell: Cell) {
        self.cells = [cell; U_COLUMNS*U_ROWS];
    }

    pub fn update_mesh(&mut self, ctx: &mut Ctx) {
        self.mesh.update_texture(ctx, bytemuck::cast_slice(&self.cells[..]));
    }

    pub fn resize_event(&mut self, width: f32, height: f32) {

    }

    pub fn draw(&mut self, ctx: &mut Ctx) {
        self.update_mesh(ctx);
        self.mesh.draw(ctx);
    }
}