#version 130

in vec2 uv;

uniform sampler2D cells;
uniform sampler2D atlas;
uniform sampler2D pal;

void main() {
    const vec2 term_size = vec2(120.0, 72.0);

    vec2 cell_pos = floor(uv / 8.0) / term_size;

    vec4 col = texture2D(atlas, uv);
    gl_FragColor = vec4(cell_pos.x, cell_pos.y, 2.0, 1.0);
}