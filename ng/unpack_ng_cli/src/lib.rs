
#![feature(arbitrary_self_types_pointers)]
use turbo_tasks::{TurboTasks, Vc};
pub mod builder;
pub mod resolve;
pub mod arguments;
use turbo_tasks_backend::{BackendOptions, TurboTasksBackend, noop_backing_storage};

pub async fn build() -> anyhow::Result<()> {
    let tt = TurboTasks::new(TurboTasksBackend::new(
        BackendOptions::default(),
        noop_backing_storage(),
    ));
    let builder = builder::UnpackBuilder::new(
        tt.clone(),
        "dist".into(),
    );
    builder.build().await?;
    
    let task = tt.spawn_root_task(|| async {
       
        Ok::<Vc<()>, _>(Default::default())
    });
    tt.wait_task_completion(task, turbo_tasks::ReadConsistency::Strong)
        .await?;
    Ok(())
}

pub fn register() {
    turbo_tasks::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}   