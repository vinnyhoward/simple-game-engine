use winit::window::{Window, WindowAttributes};
use crate::core::{Plugin, App};
use super::state::WindowState;
// use super::events::handle_window_event;

pub struct WindowConfig {
    pub attributes: WindowAttributes,
}

pub struct WindowPlugin {
    window_attributes: WindowAttributes,
    state: WindowState,
}

impl WindowPlugin {
    pub fn new() -> Self {
        let attributes = Window::default_attributes();
        Self {
            window_attributes: attributes.clone(),
            state: WindowState::new(attributes),
        }
    }

    pub fn with_attributes(window_attributes: WindowAttributes) -> Self {
        Self {
            window_attributes: window_attributes.clone(),
            state: WindowState::new(window_attributes),
        }
    }
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        // Store window configuration in app
        app.window_config = Some(WindowConfig {
            attributes: self.window_attributes.clone(),
        });
    }
} 