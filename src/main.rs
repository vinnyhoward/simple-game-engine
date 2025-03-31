mod core;
mod plugins;

use winit::event_loop::{ControlFlow, EventLoop};

use crate::core::App;
use crate::plugins::window::WindowPlugin;

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    app.add_plugin(WindowPlugin::new())
        .build();

    event_loop.run_app(&mut app)?;
    Ok(())
}