use winit::window::{Window, WindowAttributes};
use crate::plugin::Plugin;
use crate::app::App;

pub struct WindowPlugin {
    window_attributes: WindowAttributes,
}

impl WindowPlugin {
    pub fn new() -> Self {
        Self {
            window_attributes: Window::default_attributes(),
        }
    }

    pub fn with_attributes(window_attributes: WindowAttributes) -> Self {
        Self {
            window_attributes,
        }
    }
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        // Window will be created when the event loop is ready
        // We can store the attributes or other configuration here
    }
} 