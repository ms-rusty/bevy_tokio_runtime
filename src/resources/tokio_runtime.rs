#[derive(bevy_ecs::system::Resource, Default)]
pub struct TokioRuntime {
    runtime: Option<tokio::runtime::Runtime>,
}

impl TokioRuntime {
    pub fn new(&mut self, runtime: tokio::runtime::Runtime) -> Result<(), anyhow::Error> {
        if self.runtime.is_some() {
            return Err(anyhow::anyhow!("Tokio Runtime already exists."));
        }
        self.runtime = Some(runtime);

        Ok(())
    }

    pub fn spawn_task<Task, Output, Spawnable>(
        &self,
        spawnable_task: Spawnable,
    ) -> Result<tokio::task::JoinHandle<Output>, anyhow::Error>
    where
        Task: std::future::Future<Output = Output> + Send + 'static,
        Output: Send + 'static,
        Spawnable: FnOnce() -> Task + Send + 'static,
    {
        let Some(runtime) = &self.runtime else {
            return Err(anyhow::anyhow!("Tokio Runtime not found."));
        };

        Ok(runtime.spawn(spawnable_task()))
    }
}
