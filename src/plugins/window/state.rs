use winit::window::{Window, WindowAttributes};

pub struct WindowState {
    pub window: Option<Window>,
    pub attributes: WindowAttributes,
}

impl WindowState {
    pub fn new(attributes: WindowAttributes) -> Self {
        Self {
            window: None,
            attributes,
        }
    }

    pub fn default() -> Self {
        Self::new(Window::default_attributes())
    }
} 