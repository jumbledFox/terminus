#version 130

in vec2 fg_col;
in vec2 bg_col;
in vec2 tex_coord;

uniform sampler2D atlas;
uniform sampler2D pal;

void main() {
    vec4 col = texture2D(atlas, tex_coord);
    if (col.x == 1.0) {
        gl_FragColor = texture2D(pal, fg_col);
    } else {
        gl_FragColor = texture2D(pal, bg_col);
    }
    // gl_FragColor = col;
}