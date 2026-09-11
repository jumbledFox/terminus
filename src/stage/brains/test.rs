use crate::stage::{COLUMNS, ROWS, brains::Program, input::Input, screen::{Cell, Screen}};

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
            self.first = false;
            screen.fill(Cell::new(115, 5, 0, 0));
        }

        for _ in 0..10 {
            let (center_x, center_y) = (
                COLUMNS as f32 / 2.0,
                ROWS    as f32 / 2.0,
            );

            let (ch, bg) = (
                ((self.timer*3.0) as usize).rem_euclid(26) as u8 + 64,
                0b111_000_00
            );

            for i in 0..20 {
                let f = i as f32 / 1.0;
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
    
            self.timer += 1.0 / 60.0;
        }
    }
}