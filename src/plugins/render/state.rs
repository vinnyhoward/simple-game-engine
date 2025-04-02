use wgpu::{Surface, Device, Queue, SurfaceConfiguration};
use winit::window::Window;

pub struct RenderState<'window> {
    pub surface: Surface<'window>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

impl<'window> RenderState<'window> {
    pub async fn new(window: &'window Window) -> Self {
        // Create instance
        return todo!();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }
} 