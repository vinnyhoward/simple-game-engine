use winit::window::Window;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use crate::core::Plugin;

pub struct App {
    pub window: Option<Window>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn build(&mut self) {
        let plugins = std::mem::take(&mut self.plugins);
        for plugin in &plugins {
            plugin.build(self);
        }
        self.plugins = plugins;
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
} 