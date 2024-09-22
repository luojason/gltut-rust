//! Contains the standard structure for defining an [`ApplicationHandler`]
//! to feed the [`EventLoop`](winit::event_loop::EventLoop).

use glutin::{
    context::PossiblyCurrentContext,
    surface::{GlSurface, Surface, WindowSurface},
};
use winit::{application::ApplicationHandler, event::WindowEvent};

/// A helper struct for building a [`GlApp`].
pub struct GlAppBuilder<T> {
    display_fn: T,
}

fn do_nothing() {}

impl GlAppBuilder<()> {
    /// Initialize builder with default callbacks.
    ///
    /// Generally the default callbacks do nothing.
    pub fn new() -> GlAppBuilder<impl FnMut()> {
        GlAppBuilder {
            display_fn: do_nothing,
        }
    }
}

impl<T> GlAppBuilder<T> {
    /// Set a custom `display` callback. See [`GlApp`] for details.
    pub fn with_display<F: FnMut()>(self, display: F) -> GlAppBuilder<F> {
        GlAppBuilder {
            display_fn: display,
        }
    }

    /// Build the [`GlApp`].
    pub fn build(
        self,
        context: PossiblyCurrentContext,
        surface: Surface<WindowSurface>,
    ) -> GlApp<T> {
        GlApp {
            display_fn: self.display_fn,
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
///
/// Construct a `GlApp` using [`GlAppBuilder::new()`].
pub struct GlApp<T> {
    display_fn: T,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
}

impl<T> ApplicationHandler for GlApp<T>
where
    T: FnMut(),
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
                // NOTE: swap buffers is important, can get get doubled buffered surface even when single buffered is requested
                self.surface
                    .swap_buffers(&self.context)
                    .expect("failed to swap GLSurface buffers");
            }
            _ => (),
        };
    }
}
