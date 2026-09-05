#version 130

in vec2 uv;

uniform sampler2D cells;
uniform sampler2D atlas;
uniform sampler2D pal;

void main() {
    const vec2 term_size = vec2(120.0, 72.0);

    vec2 cell_pos = floor(uv / 8.0) / term_size;

    vec2 cell_uv = cell_pos + 0.5 / term_size;
    vec4 data = texture2D(cells, cell_uv);

    float character  = (data.r * 255.0); 
    float fg_i =       (data.g * 255.0); 
    float bg_i =       (data.b * 255.0); 
    int attrib =    int(data.a * 255.0); 

    vec2 cell_local = fract(uv / 8.0);

    vec2 glyph_offset = vec2(mod(character, 16.0), 15.0 - floor(character / 16.0));
    vec4 glyph = texture2D(atlas, (cell_local + glyph_offset) / 16.0);
    
    vec2 fg_uv = (vec2(mod(fg_i, 16.0), floor(fg_i / 16.0)) + 0.5) / 16.0;
    vec2 bg_uv = (vec2(mod(bg_i, 16.0), floor(bg_i / 16.0)) + 0.5) / 16.0;

    vec4 fg = texture2D(pal, fg_uv);
    vec4 bg = texture2D(pal, bg_uv);

    gl_FragColor = mix(bg, fg, glyph);
}