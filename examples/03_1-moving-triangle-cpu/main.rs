use std::time::Instant;

use gltut::glutil;
use gltut::glutil::types::*;
use gltut::glutil::{GlProgram, GlShader};

use anyhow::Context;
use gl::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: do not drop window
    let (event_loop, window, gl_context, surface) = unsafe { gltut::init_window_and_context()? };

    use_program();
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
    vtx_positions: Vec<f32>,
    position_buf_object: GLuint,
    vao: GLuint,
    start: Instant,
}

/// Renders a triangle moving counter-clockwise in a circle
impl MovingTriangle {
    fn new() -> Self {
        let position_buf_object = glutil::init_vertex_buffer(&VTX_DATA, GlBufUsage::StreamDraw);
        let vtx_positions = Vec::from(VTX_DATA);

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

        Self {
            vtx_positions,
            position_buf_object,
            vao,
            start: Instant::now(),
        }
    }

    /// Compute the triangle's current position based on the current time
    fn adjust_vtx_data(&mut self, t: u32) {
        // reset vertex data before applying new translation
        self.vtx_positions.copy_from_slice(&VTX_DATA);

        // compute translations for vertices
        const DTHETA: f32 = std::f32::consts::PI * 2.0 / (PERIOD as f32);
        let t = t % PERIOD;
        let theta = DTHETA * (t as f32);
        let dx = f32::cos(theta) * 0.5;
        let dy = f32::sin(theta) * 0.5;
        for vtx in self.vtx_positions.chunks_exact_mut(4) {
            vtx[0] += dx;
            vtx[1] += dy;
        }

        // update OpenGL buffer with the new vertex data
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.position_buf_object);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                std::mem::size_of_val(self.vtx_positions.as_slice()) as GLsizeiptr,
                self.vtx_positions.as_ptr() as *const GLvoid,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl gltut::app::GlAppDelegate for MovingTriangle {
    fn display(&mut self, app: &gltut::app::GlAppContext) {
        let t = std::time::Instant::now()
            .duration_since(self.start)
            .as_millis() as u32;
        self.adjust_vtx_data(t);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        // request another frame to create continuous animation
        app.window.request_redraw();
    }
}

const VERT_SHADER: &'static str = include_str!("./shaders/identity.vert");
const FRAG_SHADER: &'static str = include_str!("./shaders/flat-color.frag");

/// Compiles an OpenGL program to use globally
fn use_program() {
    let mut shader_list = Vec::with_capacity(2);
    shader_list.push(GlShader::compile_unwrap(GlShaderType::Vertex, &VERT_SHADER));
    shader_list.push(GlShader::compile_unwrap(
        GlShaderType::Fragment,
        &FRAG_SHADER,
    ));

    let program = GlProgram::link_unwrap(&shader_list);

    unsafe {
        gl::UseProgram(program.handle());
    }
}
