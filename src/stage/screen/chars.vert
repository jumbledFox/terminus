#version 130

in vec2 in_pos;

out vec2 uv;

uniform mat4 mvp;
uniform float scale;

void main() {
    gl_Position = mvp * vec4(in_pos, 0.0, 1.0);
    // Working but slightly inelegant way of having two different screen sizes
    // One normal, and one half-res
    uv = in_pos / scale;
}