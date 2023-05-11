use bevy_app::{App, Plugin};

pub use resources::TokioRuntime;

mod resources;

#[derive(Default)]
pub struct TokioRuntimePlugin;

impl Plugin for TokioRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TokioRuntime::default());
    }
}
