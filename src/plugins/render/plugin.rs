use crate::core::{Plugin, App};
use super::state::RenderState;
use pollster::FutureExt;

pub struct RenderPlugin;

impl RenderPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for RenderPlugin {
    fn build<'window>(&self, app: &mut App<'window>) {
        if let Some(window) = &app.window {
            let render_state = RenderState::new(window).block_on();
            // app.render_state = Some(render_state);
            todo!();
        }
    }
} 