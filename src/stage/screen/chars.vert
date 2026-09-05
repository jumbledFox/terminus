#version 130

in vec2 in_pos;

out vec2 uv;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(in_pos, 0.0, 1.0);
    uv = in_pos;
}