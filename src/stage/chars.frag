#version 130

in vec3 fg_col;
in vec3 bg_col;
in vec2 tex_coord;

uniform sampler2D tex;

void main() {
    vec4 col = texture2D(tex, tex_coord);
    if (col.x == 1.0) {
        gl_FragColor = vec4(fg_col, 1.0);
    } else {
        gl_FragColor = vec4(bg_col, 1.0);;
    }
    // gl_FragColor = col;
}