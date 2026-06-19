#version 130
in vec2 in_pos;
in vec2 inst_pos;
in int inst_data;

out vec2 tex_coord;
out vec4 fg_col;
out vec4 bg_col;

// if-- rrrg ggbb brrr gggb bbcc cccc cccc

void main() {
    int fg_invert = ((inst_data >> 31) & 0b1) == 1;
    int fg_flash  = ((inst_data >> 30) & 0b1) == 1;
    int fg_r = (inst_data >> 25) & 0b111;
    int fg_g = (inst_data >> 22) & 0b111;
    int fg_b = (inst_data >> 19) & 0b111;
    int fg_r = (inst_data >> 16) & 0b111;
    int fg_g = (inst_data >> 13) & 0b111;
    int fg_b = (inst_data >> 10) & 0b111;
    int c = inst_data & 0b1111111111;

    // todo: fancy rgb colours
    // TODO: Half-pixel offset
    // texcoord
    gl_Position = mvp * vec4(in_pos + inst_pos, 0.0, 1.0);

}