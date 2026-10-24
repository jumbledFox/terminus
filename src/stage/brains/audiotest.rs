use std::f32::consts::TAU;

use tinyaudio::{OutputDevice, OutputDeviceParameters, run_output_device};

use crate::stage::{brains::Program, input::Input, screen::{Scale, Screen, cell::Cell}};

pub struct AudioTest {
    device: OutputDevice,
}

impl AudioTest {
    pub fn new() -> Self {
        let params = OutputDeviceParameters {
            channels_count: 2,
            sample_rate: 44100,
            channel_sample_count: 512,
        };
        let device = run_output_device(params, {
            let mut clock = 0f32;
            move |data| {
                for samples in data.chunks_mut(params.channels_count) {
                    clock = (clock + 1.0) % params.sample_rate as f32;
                    let value =
                        0.2 * (clock * 440.0 * TAU / params.sample_rate as f32).sin();
                    for sample in samples {
                        *sample = value;
                    }
                }
            }
        }).expect("Failed to start audio device,,,,,");
        Self {
            device
        }
    }
}

impl Program for AudioTest {
    fn init(&mut self, screen: &mut Screen, _input: &Input) {
        screen.set_scale(Scale::Half);
    }

    fn update(&mut self, screen: &mut Screen, input: &Input) {
        for y in 0..screen.height() {
            let lightness = (((screen.height()-y-1) * 3) / screen.height()) as u16 + 1;
            let fg = (lightness << 6) + lightness / 2;
            let cell = Cell::new(6, fg, 0, 0);
            for x in 0..screen.width() {
                screen.draw_cell(x, y, cell);
            }
        }
    }
}