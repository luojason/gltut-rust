//! This module contains the implementation of most of the initialization,
//! rendering and display logic used during runtime.

use std::ffi::CString;

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
    program: gl::types::GLuint,
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

            gl::UseProgram(self.program);

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

fn init_program() -> gl::types::GLuint {
    let mut shader_list = Vec::with_capacity(2);
    shader_list.push(create_shader(gl::VERTEX_SHADER, VERT_SHADER));
    shader_list.push(create_shader(gl::FRAGMENT_SHADER, FRAG_SHADER));

    let program = create_program(&shader_list);

    shader_list
        .into_iter()
        .for_each(|shader| unsafe { gl::DeleteShader(shader) });

    return program;
}

fn create_shader(shader_type: gl::types::GLenum, shader_def: &str) -> gl::types::GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        // TODO: error handling for bad CString conversion
        let shader_def = CString::new(shader_def).unwrap();
        gl::ShaderSource(shader, 1, &shader_def.as_ptr(), std::ptr::null());

        gl::CompileShader(shader);

        // TODO: error handling on GL compilation status

        return shader;
    }
}

fn create_program(shader_list: &[gl::types::GLuint]) -> gl::types::GLuint {
    unsafe {
        let program = gl::CreateProgram();
        shader_list
            .iter()
            .for_each(|&shader| gl::AttachShader(program, shader));
        gl::LinkProgram(program);

        // TODO: clean-up error handling on GL linking status
        let mut status = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status == gl::FALSE.into() {
            let mut length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length);
            println!("message length: {}", length);

            let mut info_log: Vec<gl::types::GLchar> = [0].repeat((length).try_into().unwrap());
            gl::GetProgramInfoLog(program, length, std::ptr::null_mut(), info_log.as_mut_ptr());

            println!("c_str message: {:?}", info_log);
            // TODO: can use "from_raw_parts" conversion for this
            let info_log = info_log.into_iter().map(|v| v as u8).collect::<Vec<_>>();
            let info_log = CString::from_vec_with_nul(info_log).expect("convert to c_str");
            let info_str = info_log.clone().into_string().expect("convert to str");
            println!("utf8 string message: {}", info_str);
        }

        shader_list
            .iter()
            .for_each(|&shader| gl::DetachShader(program, shader));

        return program;
    }
}
