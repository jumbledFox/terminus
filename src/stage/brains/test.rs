use std::println;

use quad_rand::gen_range;

use crate::stage::{COLUMNS, ROWS, U_COLUMNS, U_ROWS, brains::Program, input::Input, screen::{Screen, cell::Cell}};

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
        let (s_width, s_height) = (screen.width(), screen.height());

        for x in 0..s_width {
            for y in 0..s_height {
                let x_bits = ((x * 8) / s_width)  as u16;
                let y_bits = ((y * 8) / s_height) as u16;
                let var = ((self.timer * 4.0).rem_euclid(8.0)) as u16;
    
                let (a, b, c) = match self.timer.rem_euclid(8.0) {
                    0.0..2.0 => (x_bits, y_bits, var),
                    2.0..4.0 => (y_bits, var, x_bits),
                    4.0..6.0 => (var, x_bits, y_bits),
                    _        => (var, var, var),
                };
                let bg = (a << 6) + (b << 3) + c;
    
                let cell = Cell::new(63+32, bg, 0, 0b00);
                screen.draw_cell(x, y, cell);
            }
        }

        for i in 0..U_ROWS {
            let (letter, blank, y1, y2) = (
                Cell::new(64+(i % 26) as u8, 0b111111111, 0b111000000, 0),
                Cell::new(0, 0, 0b111000000, 0),
                i + 1,
                i.saturating_sub(1),
            );
            screen.draw_cell(i, y1, blank);
            screen.draw_cell(i, y2, blank);
            screen.draw_cell(i, i, letter);
        }

        for y in 0..s_height {
            let digits = (
                 y % 10,
                (y / 10) % 10,
                (y / 100) % 10,
            );
            let chars = (
                96 + digits.0 as u8,
                match y >= 10 {
                    false => 0,
                    _ => 96 + digits.1 as u8,
                },
                match y >= 100 {
                    false => 0,
                    _ => 96 + digits.2 as u8,
                },
            );
            let (fg, bg) = (
                0b111_110_010, 0b011_001_000
            );
            let cells = (
                Cell::new(chars.0, fg, bg, 0),
                Cell::new(chars.1, fg, bg, 0),
                Cell::new(chars.2, fg, bg, 0),
            );

            screen.draw_cell(0, y, cells.2);
            screen.draw_cell(1, y, cells.1);
            screen.draw_cell(2, y, cells.0);
        }

        self.timer += 1.0 / 24.0 * 3.0;
        /*
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
                    // (y * 8) / U_ROWS,
                    // 0,
                    (x * 8) / U_COLUMNS,
                    (x * 8) / U_COLUMNS,
                    (x * 8) / U_COLUMNS,
                    // (y * 8) / U_ROWS,
                );

                // let bg_a = (x * 8) / U_COLUMNS;
                // println!("{:?}", bg_a);
                let bg = ((r << 5) + (g << 2) + (b / 2)) as u8;
                // let bg = ((0b000 << 5) + (bg_a << 2) + (bg_a / 2)) as u8;

                let cell = Cell::new(ch, 0b000_000_00, bg, 0);
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
        }*/
    }
}