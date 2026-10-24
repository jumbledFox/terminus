// TODO: Make this better - one host program that deals with changing between n loading programs.

use std::println;

use crate::stage::{brains::{audiotest::AudioTest, test::TestProgram}, input::Input, screen::Screen};

pub mod audiotest;
pub mod test;

const UPDATE_TIME: f64 = 1.0 / 60.0;

pub trait Program {
    fn init(&mut self, _screen: &mut Screen, _input: &Input) { }
    // TODO: user adjustable fps, settings,, etc,, see above ^
    // Called 20/30/60 times a frame
    fn update(&mut self, _screen: &mut Screen, _input: &Input);
}

pub struct Brains {
    program: Option<Box<dyn Program>>,
    last_time: f64,
    deltatime: f64,
    update_timer: f64,

    first: bool,
}

impl Brains {
    pub fn new() -> Self {
        Self {
            // program: Some(Box::new(TestProgram::new())),
            program: Some(Box::new(AudioTest::new())),
            last_time: miniquad::date::now(),
            deltatime: 0.0,
            update_timer: 0.0,
            first: true,
        }
    }

    pub fn update(&mut self, screen: &mut Screen, input: &Input) {
        if self.first {
            if let Some(program) = &mut self.program {
                program.init(screen, input);
            }
            self.first = false;
        }

        let now = miniquad::date::now();
        self.deltatime = now - self.last_time;
        self.last_time = now;

        self.update_timer += self.deltatime;
        while self.update_timer >= UPDATE_TIME {
            if let Some(program) = &mut self.program {
                program.update(screen, input);
            }
            self.update_timer -= UPDATE_TIME;
        }
    }
}