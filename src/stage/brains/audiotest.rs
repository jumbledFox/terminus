use std::{
    f32::consts::TAU,
    sync::mpsc::{self, Sender, TryRecvError},
};

use miniquad::KeyCode::{self, H};
use tinyaudio::{OutputDevice, OutputDeviceParameters, run_output_device};

use crate::stage::{
    brains::Program,
    input::Input,
    screen::{Scale, Screen, cell::Cell},
};

#[derive(PartialEq, Clone, Copy)]
enum Wave {
    Sine,
    Square { duty: f32 },
    Saw,
}

enum Command {
    Note(f32, Wave),
}

pub struct AudioTest {
    _device: OutputDevice,
    commands: Sender<Command>,
    wave: Wave,
    // silly dirty tests,,,
    duty_timer: f32,
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
            let mut wave = Wave::Sine;

            move |data| {
                for samples in data.chunks_mut(channels) {
                    loop {
                        match receiver.try_recv() {
                            Ok(Command::Note(hz, w)) => {
                                freq = hz;
                                wave = w;
                            }
                            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
                        }
                    }

                    clock = (clock + 1.0) % sample_rate;

                    let value = match wave {
                        Wave::Sine => (clock * freq * TAU / sample_rate).sin(),
                        Wave::Square { duty } => {
                            if (clock * freq / sample_rate) % 1.0 >= duty / 2.0 {
                                1.0
                            } else {
                                0.0
                            }
                        }
                        Wave::Saw => (clock * freq / sample_rate) % 1.0,
                    };

                    for sample in samples {
                        *sample = value;
                    }
                }
            }
        })
        .expect("Failed to start audio device,,,,,");

        Self {
            _device: device,
            commands,
            wave: Wave::Sine,
            duty_timer: 0.0,
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

        let _ = self.commands.send(Command::Note(0.0, self.wave));

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
        ]
        .iter()
        .enumerate()
        {
            if input.key_down(*key) {
                let _ = self.commands.send(Command::Note(
                    base_freq * twelth_root.powi(i as i32 + 3),
                    self.wave,
                ));
            }
        }

        self.wave = Wave::Square {
            duty: self.duty_timer.sin() * 0.4 + 0.5,
        };
        self.duty_timer += 3.0 / 24.0;

        for y in 0..screen.height() {
            let lightness = (((screen.height() - y - 1) * 5) / screen.height()) as u16 + 1;
            let fg = (lightness << 6) + lightness / 2;
            let cell = Cell::new(6, fg, 0, 0);
            for x in 0..screen.width() {
                screen.draw_cell(x, y, cell);
            }
        }

        let w_cell = Cell::new(
            // 96 + match self.wave {
            //     Wave::Sine => 1,
            //     Wave::Square => 2,
            //     Wave::Triangle => 3,
            // },
            96 + match self.wave {
                Wave::Square { duty } => (duty * 10.0) as u8,
                _ => 10,
            },
            0b111000011,
            0b111111111,
            0,
        );
        screen.draw_cell(1, 1, w_cell);
    }
}
