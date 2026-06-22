use std::io::Cursor;

use glam::{Mat4, Vec2, Vec3, vec2, vec3};
use image::ImageReader;
use miniquad::{Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, FilterMode, MipmapFilterMode, Pipeline, PipelineParams, RenderingBackend, ShaderMeta, ShaderSource, UniformBlockLayout, UniformDesc, UniformType, UniformsSource, VertexAttribute, VertexFormat, VertexStep, window};
use quad_rand::gen_range;

use crate::stage::screen::Screen;

pub mod input;
pub mod screen;

pub const COLUMNS: usize = 80;
pub const ROWS:    usize = 42;

const CHARS_TEXTURE: &[u8] = include_bytes!("../../res/chars.png");
const CHARS_VERT: &str = include_str!("chars.vert");
const CHARS_FRAG: &str = include_str!("chars.frag");

#[repr(C)]
struct Uniforms {
    mvp: Mat4,
    data: u32,
}

#[repr(C)]
struct Vertex {
    in_pos: Vec2,
}

pub struct Stage {
    ctx:      Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,

    // screen and input
    screen: Screen,

    // program
}

impl Stage {
    pub fn new() -> Self {
        let screen = Screen::new();

        // Assemble miniquad and instance buffer for screen
        // not really needed in a seperate file as that's the only rendering needed!
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        // assemble buffers
        let vertices: &[Vertex] = &[
            Vertex { in_pos: vec2( 0.0, 0.0) },
            Vertex { in_pos: vec2( 0.0, 8.0) },
            Vertex { in_pos: vec2( 8.0, 0.0) },
            Vertex { in_pos: vec2( 8.0, 8.0) },
        ];
        let indices: &[u16] = &[
            0, 1, 2,  1, 3, 2 // quad
        ];
        let geometry_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices)
        );
        let instance_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&screen.instance_data()[..]),
        );
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        // load the texture atlas
        let chars_texture = ImageReader::new(Cursor::new(CHARS_TEXTURE))
            .with_guessed_format().expect("failed to decode char atlas!")
            .decode().expect("failed to decode char atlas!")
            .flipv();
        let texture = ctx.new_texture_from_rgba8(
            chars_texture.width()  as u16,
            chars_texture.height() as u16,
            chars_texture.as_bytes()
        );
        ctx.texture_set_filter(texture, FilterMode::Nearest, MipmapFilterMode::None);

        let bindings = Bindings {
            vertex_buffers: vec![geometry_vertex_buffer, instance_vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = ctx.new_shader(
            ShaderSource::Glsl {
                vertex:   CHARS_VERT,
                fragment: CHARS_FRAG
            },
            ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![
                        UniformDesc::new("mvp",  UniformType::Mat4),
                        UniformDesc::new("data", UniformType::Int1),
                    ]
                }
            }
        ).unwrap();

        let pipeline = ctx.new_pipeline(
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("in_pos",    VertexFormat::Float2, 0),
                VertexAttribute::with_buffer("inst_pos",  VertexFormat::Float2, 1),
                VertexAttribute::with_buffer("inst_data", VertexFormat::Int1, 1),
            ],
            shader,
            PipelineParams::default(),
        );

        Self { 
            ctx, pipeline, bindings,
            screen,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        // Make sure a Vec2 is represented as #[repr(C)](f32, f32)
        assert_eq!(std::mem::size_of::<Vec2>(), 8);
        // Make sure a Vec3 is represented as #[repr(C)](f32, f32, f32)
        assert_eq!(std::mem::size_of::<Vec3>(), 12);

        // todo: only when updated
        // self.screen = Screen::new();
        self.ctx.buffer_update(
            self.bindings.vertex_buffers[1],
            BufferSource::slice(&self.screen.instance_data()[..])
        );

        // todo: no stretch, pixel perfect modes
        let proj = Mat4::orthographic_rh_gl(
            0.0, 
            COLUMNS as f32 * 8.0, 
            0.0, 
            ROWS as f32 * 8.0,
            0.001, 100.0,
        );
        let view = Mat4::look_at_rh(
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let mvp = proj * view;
        let data = 0;

        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.apply_uniforms(UniformsSource::table(&Uniforms { mvp, data }));

        self.ctx.draw(0, 6, self.screen.instance_data().len() as i32);

        self.ctx.end_render_pass();
        
        self.ctx.commit_frame();
    }
}