use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;

use super::state::WindowState;

pub fn handle_window_event(
    state: &mut WindowState,
    event_loop: &ActiveEventLoop,
    event: WindowEvent,
) {
    match event {
        WindowEvent::CloseRequested => {
            println!("The close button was pressed; stopping");
            event_loop.exit();
        },
        WindowEvent::RedrawRequested => {
            if let Some(window) = &state.window {
                window.request_redraw();
            }
        }
        _ => (),
    }
} 