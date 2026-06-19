use miniquad::*;
use stage::{Stage, COLUMNS, ROWS};

pub mod stage;

fn main() {
    quad_rand::srand(miniquad::date::now() as u64);
    let mut conf = conf::Conf::default();
    conf.window_title = "terminal emulator - jumbledFox :3".into();
    conf.window_width  = COLUMNS as i32 * 8 * 3;
    conf.window_height = ROWS    as i32 * 8 * 3;
    miniquad::start(conf, move || Box::new(Stage::new()));
}

// snake
// 3d voxel world with per-character dda raycasting, sampling textures, z buffer and billboards..?
// 0x0 - 0xF for r g and b (custom curves so 0x000 -> dark blue gray, like splash), each character 32 bits
// 0b rrrg ggbb brrr gggb bbfi --cc cccc cccc
// (foreground and background RGB, flashing, inverted, --, character index)
// functions to add/subtract from character colors, fg and bg, flashing, inverted.
// c_s("a", 0x000, 0xfff, 0b0100) // string (w/o case), fg, bg, other 4 bits
// c_i(0, 0x000, 0xfff, 0b1000) // same but with index for char