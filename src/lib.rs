use bevy_app::{App, Plugin};

pub use tokio_runtime::TokioRuntime;

mod tokio_runtime;

#[derive(Default)]
pub struct TokioRuntimePlugin;

impl Plugin for TokioRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TokioRuntime>();
    }
}
