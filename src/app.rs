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

/// A trait specifying methods required by [`GlApp`] for running a window application.
///
/// Default implementations are provided for all methods, which generally do nothing and result in a static window.
pub trait GlAppDelegate {
    /// Called when a window redraw is requested, for doing any rendering/updates needed.
    #[allow(unused_variables)]
    fn display(&mut self, app: &GlAppContext) {
        // provided implementation: do nothing
    }

    /// Called when the window has been resized, for adjusting the OpenGL viewport and the like.
    ///
    /// The new window size is provided with type [`PhysicalSize`],
    /// since `glViewPort` expects window coordinates to be specified in physical pixels.
    /// It can be converted to [`LogicalSize`](winit::dpi::LogicalSize) if needed
    /// by accessing the window provided by the [`GlAppContext`] handle.
    #[allow(unused_variables)]
    fn reshape(&mut self, app: &GlAppContext, size: &PhysicalSize<u32>) {
        set_gl_viewport(size);
    }
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

/// Allows [`GlAppDelegate`] to access handles to the various window and OpenGL related structs.
pub struct GlAppContext {
    pub window: Window,
    pub context: PossiblyCurrentContext,
    pub surface: Surface<WindowSurface>,
    _private: (), // prevent external modules from instantiating this struct
}

/// An basic implementation of [`ApplicationHandler`].
///
/// It uses a limited set of handlers to respond to some standard scenarios.
///
/// Construct a [`GlApp`] by providing an implementation of [`GlAppDelegate`],
/// or by using [`GlAppBuilder::new()`] if the callbacks are simple functions.
pub struct GlApp<T> {
    delegate: T,
    app: GlAppContext,
}

impl<T: GlAppDelegate> GlApp<T> {
    pub fn new(
        delegate: T,
        window: Window,
        context: PossiblyCurrentContext,
        surface: Surface<WindowSurface>,
    ) -> Self {
        let app = GlAppContext {
            window,
            context,
            surface,
            _private: (),
        };
        Self { delegate, app }
    }
}

impl<T: GlAppDelegate> ApplicationHandler for GlApp<T> {
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
                self.delegate.display(&self.app);

                // render the results
                unsafe {
                    gl::Flush();
                }
                self.app.window.pre_present_notify();
                // NOTE: swap buffers is important, can get get doubled buffered surface even when single buffered is requested
                self.app
                    .surface
                    .swap_buffers(&self.app.context)
                    .expect("failed to swap GLSurface buffers");
            }
            WindowEvent::Resized(size) => {
                self.app
                    .window
                    .resize_surface(&self.app.surface, &self.app.context);
                self.delegate.reshape(&self.app, &size);
            }
            _ => (),
        };
    }
}

/// A helper struct for building a basic [`GlApp`].
///
/// Can be used to pass simple callbacks for each of the methods in [`GlAppDelegate`].
/// If the callbacks need access to [`GlAppContext`] (e.g. to access the window) or if more control is needed,
/// prefer to directly implement [`GlAppDelegate`].
pub struct GlAppBuilder<T1, T2> {
    display_fn: T1,
    reshape_fn: T2,
}

impl GlAppBuilder<(), ()> {
    /// Initialize builder with default callbacks.
    ///
    /// The default callbacks have the same behavior as the default implementations in [`GlAppDelegate`].
    pub fn new() -> GlAppBuilder<impl FnMut(), impl FnMut(&PhysicalSize<u32>)> {
        GlAppBuilder {
            display_fn: do_nothing,
            reshape_fn: set_gl_viewport,
        }
    }
}

impl<T1, T2> GlAppBuilder<T1, T2> {
    /// Set a custom `display` callback. See [`GlAppDelegate`] for details.
    pub fn with_display<F: FnMut()>(self, display: F) -> GlAppBuilder<F, T2> {
        GlAppBuilder {
            display_fn: display,
            reshape_fn: self.reshape_fn,
        }
    }

    /// Set a custom `reshape` callback. See [`GlAppDelegate`] for details.
    pub fn with_reshape<F: FnMut(&PhysicalSize<u32>)>(self, reshape: F) -> GlAppBuilder<T1, F> {
        GlAppBuilder {
            display_fn: self.display_fn,
            reshape_fn: reshape,
        }
    }
}

impl<T1, T2> GlAppBuilder<T1, T2>
where
    T1: FnMut(),
    T2: FnMut(&PhysicalSize<u32>),
{
    /// Build the [`GlApp`].
    pub fn build(
        self,
        window: Window,
        context: PossiblyCurrentContext,
        surface: Surface<WindowSurface>,
    ) -> GlApp<Self> {
        GlApp::new(self, window, context, surface)
    }
}

impl<T1, T2> GlAppDelegate for GlAppBuilder<T1, T2>
where
    T1: FnMut(),
    T2: FnMut(&PhysicalSize<u32>),
{
    fn display(&mut self, _: &GlAppContext) {
        (self.display_fn)()
    }

    fn reshape(&mut self, _: &GlAppContext, size: &PhysicalSize<u32>) {
        (self.reshape_fn)(size)
    }
}
