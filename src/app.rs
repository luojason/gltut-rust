//! Contains the standard structure for defining an [`ApplicationHandler`]
//! to feed the [`EventLoop`](winit::event_loop::EventLoop).

use gl::types::*;
use glutin::{
    context::PossiblyCurrentContext,
    surface::{GlSurface, Surface, WindowSurface},
};
use glutin_winit::GlWindow;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use winit::{application::ApplicationHandler, event::WindowEvent};

/// A helper struct for building a [`GlApp`].
pub struct GlAppBuilder<T1, T2> {
    display_fn: T1,
    reshape_fn: T2,
}

fn do_nothing() {}

// glViewport takes physical pixel coordinates, so using PhysicalSize
fn set_gl_viewport(size: &PhysicalSize<u32>) {
    unsafe {
        // NOTE: not sure if this glViewport is actually doing anything
        // or if GlWindow::resize_surface already handles everything
        gl::Viewport(0, 0, size.width as GLsizei, size.height as GLsizei);
    }
}

impl GlAppBuilder<(), ()> {
    /// Initialize builder with default callbacks.
    ///
    /// Generally the default callbacks do nothing.
    pub fn new() -> GlAppBuilder<impl FnMut(), impl FnMut(&PhysicalSize<u32>)> {
        GlAppBuilder {
            display_fn: do_nothing,
            reshape_fn: set_gl_viewport,
        }
    }
}

impl<T1, T2> GlAppBuilder<T1, T2> {
    /// Set a custom `display` callback. See [`GlApp`] for details.
    pub fn with_display<F: FnMut()>(self, display: F) -> GlAppBuilder<F, T2> {
        GlAppBuilder {
            display_fn: display,
            reshape_fn: self.reshape_fn,
        }
    }

    /// Set a custom `reshape` callback. See [`GlApp`] for details.
    pub fn with_reshape<F: FnMut(&PhysicalSize<u32>)>(self, reshape: F) -> GlAppBuilder<T1, F> {
        GlAppBuilder {
            display_fn: self.display_fn,
            reshape_fn: reshape,
        }
    }

    /// Build the [`GlApp`].
    pub fn build(
        self,
        window: Window,
        context: PossiblyCurrentContext,
        surface: Surface<WindowSurface>,
    ) -> GlApp<T1, T2> {
        GlApp {
            display_fn: self.display_fn,
            reshape_fn: self.reshape_fn,
            window,
            context,
            surface,
        }
    }
}

/// An basic implementation of [`ApplicationHandler`]
/// which allows configuration of a limited set of callbacks to handle some standard scenarios.
///
/// The callbacks are:
///  - `display`: Called when a window redraw is requested, for doing any rendering/updates needed.
///  - `reshape`: Called when the window has been resized, for adjusting the OpenGL viewport and the like.
///
/// Construct a `GlApp` using [`GlAppBuilder::new()`].
pub struct GlApp<T1, T2> {
    display_fn: T1,
    reshape_fn: T2,
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
}

impl<T1, T2> ApplicationHandler for GlApp<T1, T2>
where
    T1: FnMut(),
    T2: FnMut(&PhysicalSize<u32>),
{
    fn resumed(&mut self, _: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            // stop the application once user closes the window
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // call user-specified display function
                (self.display_fn)();

                // render the results
                unsafe {
                    gl::Flush();
                }
                self.window.pre_present_notify();
                // NOTE: swap buffers is important, can get get doubled buffered surface even when single buffered is requested
                self.surface
                    .swap_buffers(&self.context)
                    .expect("failed to swap GLSurface buffers");
            }
            WindowEvent::Resized(size) => {
                self.window.resize_surface(&self.surface, &self.context);
                (self.reshape_fn)(&size);
            }
            _ => (),
        };
    }
}
