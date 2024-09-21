use anyhow::Context;
use glutin::{
    context::PossiblyCurrentContext,
    surface::{GlSurface, Surface, WindowSurface},
};
use runtime::TriangleExample;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

mod runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: do not drop _window
    let (event_loop, _window, gl_context, surface) = unsafe { gltut::init_window_and_context()? };
    let triangles = TriangleExample::new();

    // run event loop
    event_loop
        .run_app(&mut DoNothing(triangles, gl_context, surface))
        .context("failed to start event_loop")?;

    Ok(())
}

// TODO: clean this up
pub struct DoNothing(
    TriangleExample,
    PossiblyCurrentContext,
    Surface<WindowSurface>,
);

impl ApplicationHandler for DoNothing {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            // stop the application once user closes the window
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.0.display();
                // NOTE: swap buffers is important, get doubled buffered even when single buffered is requested
                self.2.swap_buffers(&self.1).unwrap();
            }
            _ => (),
        };
    }
}
