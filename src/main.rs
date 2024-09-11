use std::{ffi::CString, num::NonZeroU32};

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, NotCurrentGlContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    raw_window_handle::HasWindowHandle,
    window::{Window, WindowId},
};

#[allow(unused_variables)]
fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let window_attributes = Window::default_attributes()
        .with_inner_size(PhysicalSize::new(1000, 680))
        .with_title("OpenGL tutorial");

    let template_builder = ConfigTemplateBuilder::new().with_alpha_size(8);

    let (window, gl_config) = DisplayBuilder::new()
        .with_window_attributes(Some(window_attributes))
        .build(&event_loop, template_builder, |mut configs| {
            configs.next().unwrap()
        })
        .unwrap();
    let window = window.unwrap();
    let raw_window_handle = window.window_handle().unwrap().as_raw();
    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

    // create context
    let gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap()
    };

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(1000).unwrap(),
        NonZeroU32::new(600).unwrap(),
    );

    let surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    // make gl_context current
    let gl_context = gl_context.make_current(&surface).unwrap();

    // load OpenGL function pointers from the initialized context
    gl::load_with(|s| {
        let cstr = CString::new(s).unwrap();
        gl_display.get_proc_address(&cstr)
    });

    // run event loop
    event_loop.run_app(&mut DoNothing).unwrap();
}

pub struct DoNothing;

impl ApplicationHandler for DoNothing {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            // stop the application once user closes the window
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }
    }
}
