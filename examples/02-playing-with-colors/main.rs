use gltut::glutil;
use gltut::glutil::{GlProgram, GlShader, GlShaderType};

use anyhow::Context;
use gl::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: do not drop window
    let (event_loop, window, gl_context, surface) = unsafe { gltut::init_window_and_context()? };

    let render_ygrad = get_ygrad_render_fn();
    let render_tricolor = get_tricolor_render_fn();
    let mut app = gltut::app::GlAppBuilder::new()
        .with_display(|| {
            unsafe {
                gl::ClearColor(0.0, 0.0, 0.0, 0.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            // render both triangles
            render_ygrad();
            render_tricolor();
        })
        .build(window, gl_context, surface);

    // run event loop
    event_loop
        .run_app(&mut app)
        .context("failed to start event_loop")?;

    Ok(())
}

const YGRAD_VERT_SHADER: &'static str = include_str!("./shaders/identity.vert");
const YGRAD_FRAG_SHADER: &'static str = include_str!("./shaders/y-gradient.frag");

#[rustfmt::skip]
const YGRAD_VTX_DATA: [f32; 12] = [
    -0.25, 0.75, 0.0, 2.0,
    -0.25, -0.75, 0.0, 2.0,
    -1.75, -0.75, 0.0, 2.0,
];

fn get_ygrad_render_fn() -> impl Fn() {
    let program = init_ygrad_program();
    let vao = init_ygrad_vao();

    return move || {
        unsafe {
            gl::UseProgram(program.handle());
            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // cleanup
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    };
}

fn init_ygrad_program() -> GlProgram {
    let mut shader_list = Vec::with_capacity(2);
    shader_list.push(GlShader::compile_unwrap(
        GlShaderType::VERTEX,
        YGRAD_VERT_SHADER,
    ));
    shader_list.push(GlShader::compile_unwrap(
        GlShaderType::FRAGMENT,
        YGRAD_FRAG_SHADER,
    ));

    return GlProgram::link_unwrap(&shader_list);
}

fn init_ygrad_vao() -> GLuint {
    let position_buf_object = glutil::init_vertex_buffer(&YGRAD_VTX_DATA);
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
    return vao;
}

const TRICOLOR_VERT_SHADER: &'static str = include_str!("./shaders/multi-input.vert");
const TRICOLOR_FRAG_SHADER: &'static str = include_str!("./shaders/tricolor.frag");

#[rustfmt::skip]
const TRICOLOR_VTX_DATA: [f32; 24] = [
    // position data
    1.0,    0.5, 0.0, 2.0,
    1.5, -0.366, 0.0, 2.0,
    0.5, -0.366, 0.0, 2.0,
    // color data
    1.0,    0.0, 0.0, 1.0,
    0.0,    1.0, 0.0, 1.0,
    0.0,    0.0, 1.0, 1.0,
];

fn get_tricolor_render_fn() -> impl Fn() {
    let program = init_tricolor_program();
    let vao = init_tricolor_vao();

    return move || {
        unsafe {
            gl::UseProgram(program.handle());
            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // cleanup
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    };
}

fn init_tricolor_program() -> GlProgram {
    let mut shader_list = Vec::with_capacity(2);
    shader_list.push(GlShader::compile_unwrap(
        GlShaderType::VERTEX,
        &TRICOLOR_VERT_SHADER,
    ));
    shader_list.push(GlShader::compile_unwrap(
        GlShaderType::FRAGMENT,
        &TRICOLOR_FRAG_SHADER,
    ));

    return GlProgram::link_unwrap(&shader_list);
}

fn init_tricolor_vao() -> GLuint {
    let vertex_buf_object = glutil::init_vertex_buffer(&TRICOLOR_VTX_DATA);
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buf_object);

        gl::EnableVertexAttribArray(0); // position vertex attribute
        gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, 0 as *const GLvoid);
        gl::EnableVertexAttribArray(1); // color vertex attribute
        gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, 48 as *const GLvoid);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    return vao;
}
