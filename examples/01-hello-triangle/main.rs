use gltut::glutil::{GlProgram, GlShader, GlShaderType};

use anyhow::Context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: do not drop _window
    let (event_loop, _window, gl_context, surface) = unsafe { gltut::init_window_and_context()? };
    let triangles = TriangleExample::new();

    let mut app = gltut::app::GlAppBuilder::new()
        .with_display(|| triangles.display())
        .build(gl_context, surface);

    // run event loop
    event_loop
        .run_app(&mut app)
        .context("failed to start event_loop")?;

    Ok(())
}

/// Positions of the triangle vertices in homogeneous coordinates.
#[rustfmt::skip]
const VTX_POSITIONS: [f32; 12] = [
    0.75, 0.75, 0.0, 1.0,
    0.75, -0.75, 0.0, 1.0,
    -0.75, -0.75, 0.0, 1.0,
];

const VERT_SHADER: &'static str = include_str!("./shaders/triangle_example.vert");
const FRAG_SHADER: &'static str = include_str!("./shaders/triangle_example.frag");

/// Basic struct holding the OpenGL handles needed to represent and render a triangle.
pub struct TriangleExample {
    position_buf_object: gl::types::GLuint,
    program: GlProgram,
}

impl TriangleExample {
    pub fn new() -> Self {
        let program = init_program();
        let position_buf_object = init_vertex_buffer(&VTX_POSITIONS);

        // NOTE: this is important for some reason
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        return Self {
            position_buf_object,
            program,
        };
    }

    pub fn display(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.program.handle());

            gl::BindBuffer(gl::ARRAY_BUFFER, self.position_buf_object);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, 0 as *const gl::types::GLvoid);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // cleanup
            gl::DisableVertexAttribArray(0);
            gl::UseProgram(0);
        }
    }
}

fn init_vertex_buffer(vtx_data: &[f32]) -> gl::types::GLuint {
    let mut position_buf_object = 0;
    unsafe {
        gl::GenBuffers(1, &mut position_buf_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, position_buf_object);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(vtx_data).try_into().unwrap(),
            vtx_data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    return position_buf_object;
}

fn init_program() -> GlProgram {
    let mut shader_list = Vec::with_capacity(2);
    shader_list.push(GlShader::compile_unwrap(GlShaderType::VERTEX, VERT_SHADER));
    shader_list.push(GlShader::compile_unwrap(
        GlShaderType::FRAGMENT,
        FRAG_SHADER,
    ));

    return GlProgram::link_unwrap(&shader_list);
}