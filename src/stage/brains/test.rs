use crate::stage::{COLUMNS, ROWS, brains::Program, input::Input, screen::{Cell, Screen}};

pub struct TestProgram {
    timer: f32,
}

impl TestProgram {
    pub fn new() -> Self {
        Self { 
            timer: 0.0,
        }
    }
}

impl Program for TestProgram {
    fn update(&mut self, screen: &mut Screen, input: &Input) {
        screen.fill(Cell::new(115, 5, 0, 0));

        for _ in 0..10 {
            let (center_x, center_y) = (
                COLUMNS as f32 / 2.0,
                ROWS    as f32 / 2.0,
            );

            let (ch, bg) = (
                ((self.timer*3.0) as usize).rem_euclid(26) as u8 + 64,
                0b111_000_00
            );

            for i in 0..40 {
                let f = i as f32 / 2.0;
                let (x, y) = (
                    self.timer.sin() * f,
                    self.timer.cos() * f,
                );
                let (i_x, i_y) = (
                    (x + center_x) as usize,
                    (y + center_y) as usize,
                );
                screen.draw_cell(i_x, i_y, Cell::new(ch, 255, bg, 0));
            }
    
            self.timer += 1.0 / 60.0;
        }
    }
}