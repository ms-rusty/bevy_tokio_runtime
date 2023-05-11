use anyhow::anyhow;
use bevy_ecs::system::Resource;
use std::future::Future;
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

#[derive(Resource, Default)]
pub struct TokioRuntime {
    runtime: Option<Runtime>,
}

impl TokioRuntime {
    pub fn build(&mut self, builder: &mut Builder) -> Result<(), anyhow::Error> {
        if self.runtime.is_some() {
            return Err(anyhow!("Tokio Runtime already exists."));
        }

        self.runtime = Some(builder.build()?);

        Ok(())
    }

    pub fn spawn_task<Task, Output, Spawnable>(
        &self,
        spawnable_task: Spawnable,
    ) -> Result<JoinHandle<Output>, anyhow::Error>
    where
        Task: Future<Output = Output> + Send + 'static,
        Output: Send + 'static,
        Spawnable: FnOnce() -> Task + Send + 'static,
    {
        let Some(runtime) = &self.runtime else {
            return Err(anyhow!("Tokio Runtime not found."));
        };

        Ok(runtime.spawn(spawnable_task()))
    }
}
