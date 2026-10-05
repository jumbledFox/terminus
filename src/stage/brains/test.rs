use quad_rand::gen_range;

use crate::stage::{COLUMNS, ROWS, U_COLUMNS, U_ROWS, brains::Program, input::Input, screen::{Cell, Screen}};

pub struct TestProgram {
    timer: f32,
    first: bool,
}

impl TestProgram {
    pub fn new() -> Self {
        Self { 
            timer: 0.0,
            first: true,
        }
    }
}

impl Program for TestProgram {
    fn update(&mut self, screen: &mut Screen, input: &Input) {
        if self.first {
            for i in 0..U_COLUMNS*U_ROWS {
                let (x, y) = (
                    i % U_COLUMNS,
                    i / U_COLUMNS
                );
                let ch = match gen_range(0, 2) {
                    0 => 36,
                    _ => 41,
                };
                let bg = match (x + y) % 2 {
                    0 => 0b010_010_01,
                    _ => 0b000_000_00,
                };

                let (r, g, b) = (
                    0,
                    (x * 8) / U_COLUMNS,
                    (y * 8) / U_ROWS,
                    // (y * 8) / U_ROWS,
                );

                // let bg_a = (x * 8) / U_COLUMNS;
                // println!("{:?}", bg_a);
                let bg = ((r << 5) + (g << 2) + (b / 2)) as u8;
                // let bg = ((0b000 << 5) + (bg_a << 2) + (bg_a / 2)) as u8;

                let cell = Cell::new(ch, /*0b010_100_01*/bg, bg, 0);
                screen.draw_cell(x, y, cell);
            }
            self.first = false;
        }
        return;
        let schs = [90, 116, 106, 117];
        let screen_ch = schs[((self.timer * 0.2) as usize).rem_euclid(schs.len())];
        screen.fill(Cell::new(screen_ch, 5, 0, 0));

        for _ in 0..60 {
            let (center_x, center_y) = (
                COLUMNS as f32 / 2.0,
                ROWS    as f32 / 2.0,
            );

            let chs = [0, 1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1];
            let i = ((self.timer * 3.0) as usize).rem_euclid(chs.len());
            let ch = chs[i];
            // let bg = 0b111_000_00;

            for i in 0..20 {
                let f = i as f32;
                let (x, y) = (
                    self.timer.sin() * f,
                    self.timer.cos() * f,
                );
                let (i_x, i_y) = (
                    (x + center_x) as usize,
                    (y + center_y) as usize,
                );

                let l1 = ((i as f32 / 20.0) * 8.0) as u8;
                let l2 = 7-l1;
                let bg = l1 << 5;
                let fg = l2 << 2;
                // let ch = 6;

                screen.draw_cell(i_x, i_y, Cell::new(ch, fg, bg, 0));
            }
    
            self.timer += 1.0 / 120.0;
        }
    }
}