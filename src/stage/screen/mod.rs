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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScaleMode {
    Full, Half,
}

impl ScaleMode {
    pub fn usize(self) -> usize {
        match self {
            ScaleMode::Full => 1,
            ScaleMode::Half => 2,
        }
    }
    pub fn f32(self) -> f32 {
        match self {
            ScaleMode::Full => 1.0,
            ScaleMode::Half => 2.0,
        }
    }
}

pub struct Screen {
    mesh: ScreenMesh,
    cells: [Cell; U_COLUMNS * U_ROWS],
    scale: ScaleMode,
}

impl Screen {
    pub fn new(ctx: &mut Ctx) -> Self {
        // fill the screen with garbage :3
        let cells = [(); U_COLUMNS*U_ROWS].map(|_| Cell::random());

        let mut screen = Self {
            mesh: ScreenMesh::new(ctx),
            cells,
            scale: ScaleMode::Half,
        };

        screen.update_mesh(ctx);
        screen
    }

    pub fn xy_to_index(x: usize, y: usize) -> usize {
        x.min(U_COLUMNS-1) + y.min(U_ROWS-1) * U_COLUMNS
    }

    pub fn fill(&mut self, cell: Cell) {
        self.cells = [cell; U_COLUMNS*U_ROWS];
    }
    pub fn draw_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[Self::xy_to_index(x, y)] = cell;
    }

    // pub fn resize_event(&mut self, width: f32, height: f32) { }

    pub fn scale(&self) -> ScaleMode {
        self.scale
    }
    pub fn set_scale(&mut self, scale: ScaleMode) {
        self.scale = scale;
    }

    pub fn width(&self) -> usize {
        U_COLUMNS / self.scale().usize()
    }
    pub fn height(&self) -> usize {
        U_ROWS / self.scale().usize()
    }

    pub fn update_mesh(&mut self, ctx: &mut Ctx) {
        self.mesh.update_texture(ctx, bytemuck::cast_slice(&self.cells[..]));
    }
    
    pub fn draw(&mut self, ctx: &mut Ctx) {
        self.update_mesh(ctx);
        self.mesh.draw(ctx, self.scale.f32());
    }
}