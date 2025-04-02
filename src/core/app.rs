use winit::window::Window;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use crate::core::Plugin;
use crate::plugins::window::WindowConfig;
use crate::plugins::render::RenderState;

pub struct App<'window> {
    pub window: Option<Window>,
    pub window_config: Option<WindowConfig>,
    pub render_state: Option<RenderState<'window>>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl<'window> App<'window> {
    pub fn new() -> Self {
        Self {
            window: None,
            window_config: None,
            render_state: None,
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

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attributes = self.window_config
                .as_ref()
                .map(|config| config.attributes.clone())
                .unwrap_or_else(Window::default_attributes);
            
            self.window = Some(event_loop.create_window(attributes).unwrap());
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