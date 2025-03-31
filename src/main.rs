mod core;
mod plugins;

use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use winit::dpi::{PhysicalSize, Size};

use crate::core::App;
use crate::plugins::window::WindowPlugin;

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    // Create custom window attributes
    let mut attributes = Window::default_attributes();
    attributes.title = "My Game Engine".into();
    attributes.inner_size = Some(Size::Physical(PhysicalSize::new(800, 600)));
    attributes.resizable = true;

    let mut app = App::new();
    app.add_plugin(WindowPlugin::with_attributes(attributes))
        .build();

    event_loop.run_app(&mut app)?;
    Ok(())
}