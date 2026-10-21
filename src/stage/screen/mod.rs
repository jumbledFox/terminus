// Screen struct that stores mesh, duplicate array, and has functions for drawing shapes
// keeps track of dirty rows to be updated / redrawn,

use miniquad::window::screen_size;
use quad_rand::gen_range;

use crate::stage::{COLUMNS, Ctx, ROWS, U_COLUMNS, U_ROWS, screen::{cell::Cell, mesh::ScreenMesh}};

pub mod cell;
mod mesh;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ViewMode {
    Stretch,
    Fit,
    PixelPerfect,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Scale {
    Full, Half,
}

impl Scale {
    pub fn usize(self) -> usize {
        match self {
            Scale::Full => 1,
            Scale::Half => 2,
        }
    }
    pub fn f32(self) -> f32 {
        match self {
            Scale::Full => 1.0,
            Scale::Half => 2.0,
        }
    }
}

pub struct Screen {
    mesh: ScreenMesh,
    cells: [Cell; U_COLUMNS * U_ROWS],
    scale: Scale,
    view_mode: ViewMode,
    lrbt: (f32, f32, f32, f32), // left, right, top, bottom
}

impl Screen {
    pub fn new(ctx: &mut Ctx) -> Self {
        // fill the screen with garbage :3
        let cells = [(); U_COLUMNS*U_ROWS].map(|_| Cell::random());

        let mut screen = Self {
            mesh: ScreenMesh::new(ctx),
            cells,
            scale: Scale::Half,
            view_mode: ViewMode::Fit,
            lrbt: (0.0, 0.0, 0.0, 0.0),
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

    pub fn resize(&mut self, size: Option<(f32, f32)>) {
        let (sw, sh) = size.unwrap_or_else(|| screen_size());
        let (vw, vh) = (COLUMNS as f32 * 8.0, ROWS as f32 * 8.0);
        
        // derived from notes in my silly notebook,,,

        self.lrbt = match self.view_mode {
            ViewMode::Fit => match vw/vh < sw/sh {
                // if the screen is wider than the view
                true  => {
                    let x = ((sw/sh)*vh - vw) / 2.0;
                    (-x, vw+x, 0.0, vh)
                },
                // if the screen is taller than the view
                false => {
                    let y = ((sh/sw)*vw - vh) / 2.0;
                    (0.0, vw, -y, vh+y)
                },
            }
            ViewMode::Stretch | _ => (
                0.0, 
                COLUMNS as f32 * 8.0, 
                0.0, 
                ROWS as f32 * 8.0),
        };
    }

    pub fn scale(&self) -> Scale {
        self.scale
    }
    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }
    pub fn test_view(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::Stretch => ViewMode::Fit,
            ViewMode::Fit => ViewMode::PixelPerfect,
            ViewMode::PixelPerfect => ViewMode::Stretch,
        };
        self.resize(None);
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
        self.mesh.draw(ctx, self.lrbt, self.scale.f32());
    }
}