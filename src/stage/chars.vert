#version 130
in vec2 in_pos;

in vec2 inst_pos;
in float inst_fg;
in float inst_bg;
// cccc.fi
in float inst_char;

out vec2 tex_coord;
out vec2 fg_col;
out vec2 bg_col;

// uniform uint data;
uniform mat4 mvp;

void main() {
    // TODO: Half-pixel offset
    // todo: mod
    tex_coord = (vec2(float(int(inst_char) % 16), 15.0 - float(int(inst_char) / 16)) + (in_pos / 8.0)) / 16.0;
    fg_col = (vec2(mod(inst_fg, 100.0), trunc(inst_fg / 100.0)) + 0.5) / vec2(100.0, 10.0);
    bg_col = (vec2(mod(inst_bg, 100.0), trunc(inst_bg / 100.0)) + 0.5) / vec2(100.0, 10.0);

    gl_Position = mvp * vec4(in_pos + inst_pos, 0.0, 1.0);
}

/*
// rrrg ggbb brrr gggb bbcc cccc cif- ---- 
    // float fg_r = 1.0; 
    // float fg_g = 1.0; 
    // float fg_b = 1.0; 
    // float bg_r = 0.5;
    // float bg_g = 0.5;
    // float bg_b = 0.5;
    // uint c = 64u;
    // if (inst_data == 0x40C00000) {
    //     c = 65u;
    // }
    // bool invert = false;
    // bool flash  = false;
    // uint inst_data = 3766476800u; 
    // float fg_r = float((inst_data >> 29u) & 7u) / 7.0; //0b111
    // float fg_g = float((inst_data >> 26u) & 7u) / 7.0; //0b111
    // float fg_b = float((inst_data >> 23u) & 7u) / 7.0; //0b111
    // float bg_r = float((inst_data >> 20u) & 7u) / 7.0; //0b111
    // float bg_g = float((inst_data >> 17u) & 7u) / 7.0; //0b111
    // float bg_b = float((inst_data >> 14u) & 7u) / 7.0; //0b111
    // uint c = ((inst_data >> 7u) & 127u); //0b111 1111;
    // bool invert = ((inst_data >> 6u) & 1u) == 1u;
    // bool flash  = ((inst_data >> 5u) & 1u) == 1u;

    // todo: rgb colors ramped to force palette

    // fg_col = vec3(fg_r * 0.81 + 0.19, fg_g * 0.9 + 0.1, fg_b * 0.88 + 0.12);
    // bg_col = vec3(bg_r * 0.81 + 0.19, bg_g * 0.9 + 0.1, bg_b * 0.88 + 0.12);

    // bool flashing = (data & 1u) == 1u;
    // if (invert ^^ (flashing && flash)) {
    //     vec3 temp = vec3(fg_col);
    //     fg_col = bg_col;
    //     bg_col = temp;
    // }
    */