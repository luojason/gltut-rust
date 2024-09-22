use anyhow::Context;
use runtime::TriangleExample;

mod runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: do not drop _window
    let (event_loop, _window, gl_context, surface) = unsafe { gltut::init_window_and_context()? };
    let triangles = TriangleExample::new();

    let mut app = gltut::app::GlAppBuilder::new()
        .with_display(|| triangles.display())
        .build(gl_context, surface);

    // run event loop
    event_loop
        .run_app(&mut app)
        .context("failed to start event_loop")?;

    Ok(())
}
