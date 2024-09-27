//! Contains type-safe wrappers over some of OpenGL's enum types.
//!
//! This makes it less likely to input a wrong value in OpenGL APIs when `GLenum` is expected.

use gl::types::*;

// TODO: explore if this can be generated with a proc-macro

/// Type-safe wrapper over `GLenum` which can only represent valid shader types.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GlShaderType {
    Vertex,
    Fragment,
}

impl GlShaderType {
    /// Convert to the underlying `GLenum` value.
    pub const fn value(&self) -> GLenum {
        match self {
            GlShaderType::Vertex => gl::VERTEX_SHADER,
            GlShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

/// Type-safe wrapper over `GLenum` which can only represent valid buffer usage modes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GlBufUsage {
    StaticDraw,
    StreamDraw,
}

impl GlBufUsage {
    /// Convert to the underlying `GLenum` value.
    pub const fn value(&self) -> GLenum {
        match self {
            GlBufUsage::StaticDraw => gl::STATIC_DRAW,
            GlBufUsage::StreamDraw => gl::STREAM_DRAW,
        }
    }
}
