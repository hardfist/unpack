use std::future::Future;

use tokio::{task::JoinHandle, task_local};

task_local! {
    pub static COMPILER_ID: u32;
}

fn spawn_with_id<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let compiler_id = COMPILER_ID.get();

    tokio::spawn(COMPILER_ID.scope(compiler_id, future))
}
