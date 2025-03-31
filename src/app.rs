use winit::window::Window;
use crate::plugin::Plugin;

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