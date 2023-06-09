use bevy_ecs::system::Resource;
use std::future::Future;
use tokio::{runtime::Runtime, task::JoinHandle};

#[derive(Resource, Default)]
pub struct TokioRuntime {
    runtime: Option<Runtime>,
}

impl TokioRuntime {
    pub fn new(&mut self, runtime: Runtime) {
        if self.runtime.is_some() {
            panic!("Tokio Runtime already exists.");
        }

        self.runtime = Some(runtime);
    }

    pub fn spawn_task<Task>(&self, future: Task) -> JoinHandle<Task::Output>
    where
        Task: Future + Send + 'static,
        Task::Output: Send + 'static,
    {
        let Some(runtime) = &self.runtime else {
            panic!("Tokio Runtime not found.");
        };

        runtime.spawn(future)
    }

    pub fn get_runtime(&self) -> &Option<Runtime> {
        &self.runtime
    }
}
