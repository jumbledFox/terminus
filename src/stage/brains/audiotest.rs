use std::{f32::consts::TAU, sync::mpsc::{self, Sender, TryRecvError}};

use miniquad::KeyCode::{self, H};
use tinyaudio::{OutputDevice, OutputDeviceParameters, run_output_device};

use crate::stage::{brains::Program, input::Input, screen::{Scale, Screen, cell::Cell}};

enum Command {
    Note(f32),
}

pub struct AudioTest {
    _device: OutputDevice,
    commands: Sender<Command>
}

impl AudioTest {
    pub fn new() -> Self {
        let params = OutputDeviceParameters {
            channels_count: 2,
            sample_rate: 44100,
            channel_sample_count: 512,
        };

        let channels = params.channels_count;
        let sample_rate = params.sample_rate as f32;
        
        let (commands, receiver) = mpsc::channel();

        let device = run_output_device(params, {
            let mut clock = 0f32;
            let mut freq = 0.0;

            move |data| {
                for samples in data.chunks_mut(channels) {
                    loop {
                        match receiver.try_recv() {
                            Ok(Command::Note(hz)) => freq = hz,
                            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
                        }
                    }

                    clock = (clock + 1.0) % sample_rate;
                    let value =
                        0.2 * if (clock * freq * TAU / sample_rate).sin() >= 0.5 { 1.0 } else { 0.0 };
                    for sample in samples {
                        *sample = value;
                    }
                }
            }
        }).expect("Failed to start audio device,,,,,");

        Self {
            _device: device,
            commands
        }
    }
}

impl Program for AudioTest {
    fn init(&mut self, screen: &mut Screen, _input: &Input) {
        screen.set_scale(Scale::Half);
    }

    fn update(&mut self, screen: &mut Screen, input: &Input) {
        let base_freq = 110.0; //A2
        let twelth_root = 2.0f32.powf(1.0 / 12.0);

        let _ = self.commands.send(Command::Note(0.0));

        for (i, key) in [
            KeyCode::Z,    // C
            KeyCode::S,    // C#
            KeyCode::X,    // D
            KeyCode::D,    // D#
            KeyCode::C,    // E
            KeyCode::V,    // F
            KeyCode::G,    // F#
            KeyCode::B,    // G
            KeyCode::H,    // G#

            KeyCode::N,    // A
            KeyCode::J,    // A#
            KeyCode::M,    // B
            KeyCode::Q,    // C
            KeyCode::Key2, // C#
            KeyCode::W,    // D
            KeyCode::Key3, // D#
            KeyCode::E,    // E
            KeyCode::R,    // F
            KeyCode::Key5, // F#
            KeyCode::T,    // G
            KeyCode::Key6, // G#

            KeyCode::Y,    // A
            KeyCode::Key7, // A#
            KeyCode::U,    // B
            KeyCode::I,    // C
            KeyCode::Key9, // C#
            KeyCode::O,    // D
            KeyCode::Key0, // D#
            KeyCode::P,    // E
        ].iter().enumerate() {
            if input.key_down(*key) {
                let _ = self.commands.send(Command::Note(base_freq * twelth_root.powi(i as i32 + 3)));
            }
        }

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