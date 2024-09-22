//! Contains utility functions for some common OpenGL operations.

use std::ffi;

use thiserror::Error;

/// Type-safe wrapper over `GLenum` which can only represent valid shader types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GlShaderType {
    /// Maps to ` GL_VERTEX_SHADER`.
    VERTEX,
    /// Maps to ` GL_FRAGMENT_SHADER`.
    FRAGMENT,
}

impl GlShaderType {
    /// Convert to the underlying `GLenum` value.
    pub const fn value(&self) -> gl::types::GLenum {
        match self {
            GlShaderType::VERTEX => gl::VERTEX_SHADER,
            GlShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
        }
    }
}

/// An RAII struct managing the lifetime of a shader object.
///
/// It represents a uniquely owned shader, hence is not [`Copy`] or [`Clone`].
#[derive(Debug)]
pub struct GlShader {
    id: gl::types::GLuint,
}

impl GlShader {
    /// Creates a shader object from the provided GLSL source string.
    pub fn compile(shader_type: GlShaderType, source: &str) -> Result<Self, GlShaderError> {
        unsafe {
            let shader = gl::CreateShader(shader_type.value());
            // Wrap shader now so it is dropped if failure occurs later in the method
            let result = Self { id: shader };

            let shader_def = ffi::CString::new(source)?;
            gl::ShaderSource(shader, 1, &shader_def.as_ptr(), std::ptr::null());

            gl::CompileShader(shader);

            let mut status = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
            if status == gl::FALSE.into() {
                let msg = result.get_shader_info_log();
                return Err(GlShaderCompileError {
                    shader_type: shader_type.value(),
                    msg,
                })?;
            }

            return Ok(result);
        }
    }

    /// Like [`Self::compile()`] but panics with error message on failure.
    pub fn compile_unwrap(shader_type: GlShaderType, source: &str) -> Self {
        Self::compile(shader_type, source)
            .inspect_err(|e| eprintln!("failed to compile shader: {}", e))
            .unwrap()
    }

    /// Get the `GLuint` this struct is wrapping.
    #[inline]
    pub fn handle(&self) -> gl::types::GLuint {
        self.id
    }

    /// Helper function to call `glGetShaderInfoLog` and allocate space to store the string.
    pub fn get_shader_info_log(&self) -> ffi::CString {
        let mut length: gl::types::GLint = 0;

        unsafe {
            gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut length);
        }

        // Allocate buffer and populate it with the log
        let mut info_log: Vec<gl::types::GLchar> = vec![0; length as usize];
        unsafe {
            gl::GetShaderInfoLog(self.id, length, std::ptr::null_mut(), info_log.as_mut_ptr());
        }

        // Cast the c_str buffer to unsigned byte type
        // Prevent old Vec from dropping since onwership of contents will be transferred to the new, casted Vec
        let mut v = std::mem::ManuallyDrop::new(info_log);
        let info_log = {
            let ptr = v.as_mut_ptr() as *mut u8;
            let len = v.len();
            let cap = v.capacity();
            // SAFETY: the raw parts were extract from a valid vector, and i8/u8 pointer cast is valid
            unsafe { Vec::from_raw_parts(ptr, len, cap) }
        };

        return ffi::CString::from_vec_with_nul(info_log).unwrap();
    }
}

impl Drop for GlShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

/// An RAII struct managing the lifetime of a program object.
///
/// It represents a uniquely owned program, hence is not [`Copy`] or [`Clone`].
pub struct GlProgram {
    id: gl::types::GLuint,
}

impl GlProgram {
    /// Creates a program object by linking the provided [`GlShader`] objects.
    pub fn link(shaders: &[GlShader]) -> Result<Self, GlProgramLinkError> {
        unsafe {
            let program = gl::CreateProgram();
            // Wrap program now so it is dropped if failure occurs later in the method
            let result = Self { id: program };

            shaders
                .iter()
                .map(GlShader::handle)
                .for_each(|shader| gl::AttachShader(program, shader));

            gl::LinkProgram(program);

            let mut status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
            if status == gl::FALSE.into() {
                let msg = result.get_program_info_log();
                return Err(GlProgramLinkError { msg });
            }

            return Ok(result);
        }
    }

    /// Like [`Self::link()`] but panics with error message on failure.
    pub fn link_unwrap(shaders: &[GlShader]) -> Self {
        Self::link(shaders)
            .inspect_err(|e| eprintln!("failed to link program: {}", e))
            .unwrap()
    }

    /// Get the `GLuint` this struct is wrapping.
    #[inline]
    pub fn handle(&self) -> gl::types::GLuint {
        self.id
    }

    /// Helper function to call `glGetProgramInfoLog` and allocate space to store the string.
    pub fn get_program_info_log(&self) -> ffi::CString {
        let mut length: gl::types::GLint = 0;

        unsafe {
            gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut length);
        }

        // Allocate buffer and populate it with the log
        let mut info_log: Vec<gl::types::GLchar> = vec![0; length as usize];
        unsafe {
            gl::GetProgramInfoLog(self.id, length, std::ptr::null_mut(), info_log.as_mut_ptr());
        }

        // Cast the c_str buffer to unsigned byte type
        // Prevent old Vec from dropping since onwership of contents will be transferred to the new, casted Vec
        let mut v = std::mem::ManuallyDrop::new(info_log);
        let info_log = {
            let ptr = v.as_mut_ptr() as *mut u8;
            let len = v.len();
            let cap = v.capacity();
            // SAFETY: the raw parts were extract from a valid vector, and i8/u8 pointer cast is valid
            unsafe { Vec::from_raw_parts(ptr, len, cap) }
        };

        return ffi::CString::from_vec_with_nul(info_log).unwrap();
    }
}

impl Drop for GlProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

/// Errors that can occur when compiling a [`GlShader`].
#[derive(Debug, Error)]
pub enum GlShaderError {
    #[error("failed to parse source string: {0}")]
    CStrError(#[from] ffi::NulError),
    #[error(transparent)]
    CompileError(#[from] GlShaderCompileError),
}

#[derive(Debug, Error)]
#[error(
    "compiler error in {} shader: {}",
    get_shader_type(*.shader_type),
    .msg.to_string_lossy()
)]
pub struct GlShaderCompileError {
    shader_type: gl::types::GLenum,
    msg: ffi::CString,
}

#[derive(Debug, Error)]
#[error(
    "linker error: {}",
    .msg.to_string_lossy()
)]
pub struct GlProgramLinkError {
    msg: ffi::CString,
}

#[inline]
const fn get_shader_type(shader_type: gl::types::GLenum) -> &'static str {
    match shader_type {
        gl::VERTEX_SHADER => "vertex",
        gl::GEOMETRY_SHADER => "geometry",
        gl::FRAGMENT_SHADER => "fragment",
        _ => "unknown",
    }
}
