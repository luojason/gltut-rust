//! Contains utility functions for some common OpenGL operations.

mod shader;
pub use shader::*;

/// Initializes a GL buffer to store floats and populates it with the provided data.
///
/// Returns the generated buffer object name.
pub fn init_vertex_buffer(vtx_data: &[f32]) -> gl::types::GLuint {
    let mut vtx_buffer_object = 0;
    unsafe {
        gl::GenBuffers(1, &mut vtx_buffer_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, vtx_buffer_object);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(vtx_data) as gl::types::GLsizeiptr,
            vtx_data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    return vtx_buffer_object;
}
