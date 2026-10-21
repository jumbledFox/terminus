#version 130

in vec2 uv;

uniform sampler2D cells;
uniform sampler2D atlas;
uniform sampler2D pal;

void main() {
    // TODO: A nice way of two-mode res
    const vec2 term_size = vec2(120.0, 72.0);
    vec2 uv = uv / 2.0 + vec2(0.0, 72.0 * 4.0);

    vec2 cell_pos = floor(uv / 8.0) / term_size;

    vec2 cell_uv = (cell_pos + 0.5 / term_size) * vec2(1.0, -1.0) + vec2(0.0, 1.0);
    vec4 data = texture2D(cells, cell_uv);

    // TODO: Make this an int too?!
    float character  = floor(data.r * 255.0 + 0.5);
    int fg8    = int(data.g * 255.0 + 0.5);
    int bg8    = int(data.b * 255.0 + 0.5);
    int attrib = int(data.a * 255.0 + 0.5);

    // append bits 6 and 7 to colors
    int fgIndex = fg8 | (((attrib >> 7) & 1) << 8);
    int bgIndex = bg8 | (((attrib >> 6) & 1) << 8);

    // int flashMode = attrib & 3;

    ivec2 fgCoord = ivec2(fgIndex % 64, fgIndex / 64);
    ivec2 bgCoord = ivec2(bgIndex % 64, bgIndex / 64);

    vec4 fg = texelFetch(pal, fgCoord, 0);
    vec4 bg = texelFetch(pal, bgCoord, 0);

    vec2 cell_local = fract(uv / 8.0);
    vec2 glyph_offset = vec2(mod(character, 16.0), 15.0 - floor(character / 16.0));
    vec4 glyph = texture2D(atlas, (cell_local + glyph_offset) / 16.0);

    gl_FragColor = mix(bg, fg, glyph);
}