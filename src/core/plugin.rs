use super::app::App;

pub trait Plugin {
    fn build<'window>(&self, app: &mut App<'window>);
} 