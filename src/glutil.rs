//! Contains utility functions for some common OpenGL operations.

use gl::types::*;

mod shader;
pub use shader::*;

pub mod types;
use types::*;

/// Initializes a GL buffer to store floats and populates it with the provided data.
///
/// Returns the generated buffer object name.
pub fn init_vertex_buffer(vtx_data: &[f32], usage: GlBufUsage) -> GLuint {
    let mut vtx_buffer_object = 0;
    unsafe {
        gl::GenBuffers(1, &mut vtx_buffer_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, vtx_buffer_object);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(vtx_data) as GLsizeiptr,
            vtx_data.as_ptr() as *const GLvoid,
            usage.value(),
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    return vtx_buffer_object;
}
