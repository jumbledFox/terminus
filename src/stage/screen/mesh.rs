use std::io::Cursor;

use glam::{Mat4, Vec2, Vec3, vec2, vec3};
use image::ImageReader;
use miniquad::{Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, FilterMode, MipmapFilterMode, Pipeline, PipelineParams, ShaderMeta, ShaderSource, TextureFormat, TextureId, TextureKind, TextureParams, TextureWrap, UniformBlockLayout, UniformDesc, UniformType, UniformsSource, VertexAttribute, VertexFormat, window::screen_size};

use crate::stage::{COLUMNS, Ctx, ROWS, U_COLUMNS, U_ROWS, screen::Cell};

const CHARS_TEXTURE: &[u8] = include_bytes!("../../../res/chars.png");
const PAL_TEXTURE: &[u8]   = include_bytes!("../../../res/pal.png");
const CHARS_VERT: &str = include_str!("chars.vert");
const CHARS_FRAG: &str = include_str!("chars.frag");

#[repr(C)]
struct Uniforms {
    mvp: Mat4,
    scale: f32,
    // data: u32,
}

#[repr(C)]
struct Vertex {
    in_pos: Vec2,
}

pub struct ScreenMesh {
    bindings: Bindings,
    pipeline: Pipeline,
    cells_texture: TextureId,
}

impl ScreenMesh {
    pub fn new(ctx: &mut Ctx) -> Self {
        // assemble buffers
        let (col, row) = (
            COLUMNS as f32 * 8.0, ROWS as f32 * 8.0
        );
        let vertices: &[Vertex] = &[
            Vertex { in_pos: vec2( 0.0, 0.0) },
            Vertex { in_pos: vec2( 0.0, row) },
            Vertex { in_pos: vec2( col, 0.0) },
            Vertex { in_pos: vec2( col, row) },
        ];
        let indices: &[u16] = &[
            0, 1, 2,  1, 3, 2 // quad
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices)
        );
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        // create cells texture
        let cells_texture = ctx.new_render_texture(TextureParams {
            kind:   TextureKind::Texture2D,
            format: TextureFormat::RGBA8,
            wrap:   TextureWrap::Clamp,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::None,
            width:   COLUMNS as u32,
            height:  ROWS    as u32,
            allocate_mipmaps: false,
            sample_count: 1,
        });

        // Load the character atlas and palette
        let mut load_texture = |bytes: &[u8], flip_v: bool| -> TextureId {
            let mut t = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format().expect("failed to decode char atlas!")
                .decode().expect("failed to decode char atlas!");
            if flip_v {
                t = t.flipv()
            }
            let texture = ctx.new_texture_from_rgba8(
                t.width()  as u16,
                t.height() as u16,
                t.as_bytes()
            );
            ctx.texture_set_filter(texture, FilterMode::Nearest, MipmapFilterMode::None);
            texture
        };
        let chars = load_texture(CHARS_TEXTURE, true);
        let pal = load_texture(PAL_TEXTURE, false);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![cells_texture, chars, pal],
        };

        let shader = ctx.new_shader(
            ShaderSource::Glsl {
                vertex:   CHARS_VERT,
                fragment: CHARS_FRAG
            },
            ShaderMeta {
                images: vec![
                    "cells".to_string(),
                    "atlas".to_string(),
                    "pal".to_string(),
                ],
                uniforms: UniformBlockLayout {
                    uniforms: vec![
                        UniformDesc::new("mvp",   UniformType::Mat4),
                        UniformDesc::new("scale", UniformType::Float1),
                    ]
                }
            }
        ).unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        Self { bindings, pipeline, cells_texture }
    }

    pub fn update_texture(&mut self, ctx: &mut Ctx, bytes: &[u8]) {
        ctx.texture_update(self.cells_texture, bytes);
    }

    pub fn draw(&self, ctx: &mut Ctx, lrbt: (f32, f32, f32, f32), scale: f32) {
        let (l, r, b, t) = lrbt;
        let proj = Mat4::orthographic_rh_gl(
            l, r, b, t,
            0.001, 100.0,
        );
        // todo: no stretch, pixel perfect modes
        // let proj = Mat4::orthographic_rh_gl(
        //     0.0, 
        //     COLUMNS as f32 * 8.0, 
        //     0.0, 
        //     ROWS as f32 * 8.0,
        //     0.001, 100.0,
        // );
        let view = Mat4::look_at_rh(
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let mvp = proj * view;

        ctx.begin_default_pass(Default::default());
        ctx.clear(Some((0.3, 1.0, 0.1, 1.0)), None, None);
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(UniformsSource::table(&Uniforms { mvp, scale }));
        ctx.draw(0, 6, 1);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}