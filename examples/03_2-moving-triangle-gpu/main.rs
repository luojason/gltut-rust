use std::ffi::CString;
use std::time::Instant;

use gltut::glutil;
use gltut::glutil::types::*;
use gltut::glutil::{GlProgram, GlShader};

use anyhow::Context;
use gl::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: do not drop window
    let (event_loop, window, gl_context, surface) = unsafe { gltut::init_window_and_context()? };

    let triangle = MovingTriangle::new();
    let mut app = gltut::app::GlApp::new(triangle, window, gl_context, surface);

    // run event loop
    event_loop
        .run_app(&mut app)
        .context("failed to start event_loop")?;

    Ok(())
}

#[rustfmt::skip]
const VTX_DATA: [f32; 12] = [
    0.25, 0.25, 0.0, 1.0,
	0.25, -0.25, 0.0, 1.0,
	-0.25, -0.25, 0.0, 1.0,
];

/// How many milliseconds to complete one revolution
const PERIOD: u32 = 2048;

struct MovingTriangle {
    program: GlProgram,
    /// Location for updating the vertex translation uniform
    offset_location: GLint,
    vao: GLuint,
    start: Instant,
}

/// Renders a triangle moving counter-clockwise in a circle
impl MovingTriangle {
    fn new() -> Self {
        let (program, offset_location) = init_program();
        let vao = init_vao();
        Self {
            program,
            offset_location,
            vao,
            start: Instant::now(),
        }
    }
}

impl gltut::app::GlAppDelegate for MovingTriangle {
    fn display(&mut self, app: &gltut::app::GlAppContext) {
        let t = std::time::Instant::now()
            .duration_since(self.start)
            .as_millis() as u32;
        let (dx, dy) = get_offset(t);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.program.handle());

            // set new offset
            gl::Uniform2f(self.offset_location, dx, dy);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        // request another frame to create continuous animation
        app.window.request_redraw();
    }
}

fn init_vao() -> GLuint {
    let position_buf_object = glutil::init_vertex_buffer(&VTX_DATA, GlBufUsage::StaticDraw);
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, position_buf_object);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, 0 as *const GLvoid);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    vao
}

const VERT_SHADER: &'static str = include_str!("./shaders/translate.vert");
const FRAG_SHADER: &'static str = include_str!("./shaders/cycle-color.frag");

/// Compiles an OpenGL program to use globally
fn init_program() -> (GlProgram, GLint) {
    let mut shader_list = Vec::with_capacity(2);
    shader_list.push(GlShader::compile_unwrap(GlShaderType::Vertex, &VERT_SHADER));
    shader_list.push(GlShader::compile_unwrap(
        GlShaderType::Fragment,
        &FRAG_SHADER,
    ));

    let offset_name = CString::new("offset").unwrap();
    let program = GlProgram::link_unwrap(&shader_list);
    let offset_location = unsafe { gl::GetUniformLocation(program.handle(), offset_name.as_ptr()) };

    (program, offset_location)
}

/// Computes the offset based on the provided time
fn get_offset(t: u32) -> (f32, f32) {
    const DTHETA: f32 = std::f32::consts::PI * 2.0 / (PERIOD as f32);
    let t = t % PERIOD;
    let theta = DTHETA * (t as f32);
    let dx = f32::cos(theta) * 0.5;
    let dy = f32::sin(theta) * 0.5;

    (dx, dy)
}
