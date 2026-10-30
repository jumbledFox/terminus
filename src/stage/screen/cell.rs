use quad_rand::gen_range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Cell {
    ch: u8,
    fg: u8,     //0brr_ggg_bbb
    bg: u8,     // ""
    attrib: u8, // fg_r bg_r xxxx flash flash
}

impl Cell {
    pub fn random() -> Self {
        Self {
            ch: gen_range(0, 255),
            fg: gen_range(0, 255),
            bg: gen_range(0, 255),
            attrib: gen_range(0, 255),
        }
    }

    pub fn new(ch: u8, fg: u16, bg: u16, flash: u8) -> Self {
        let f = (fg & 0b1111_1111) as u8;
        let b = (bg & 0b1111_1111) as u8;

        let fg_r = ((fg >> 8) & 1) as u8;
        let bg_r = ((bg >> 8) & 1) as u8;

        let attrib = (flash & 0b11) + (fg_r << 7) + (bg_r << 6);

        Self {
            ch,
            fg: f,
            bg: b,
            attrib,
        }
    }

    pub fn char_to_index(c: char) -> u8 {
        match c {
            'A'..='Z' => c as u8 - 65 + 64,
            'a'..='z' => c as u8 - 97 + 64,
            '0'..='9' => c as u8 - 48 + 96,
            _ => 0,
        }
    }
}
