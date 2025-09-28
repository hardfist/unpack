#![feature(arbitrary_self_types_pointers)]

use turbo_tasks::{TurboTasks, Vc};
use turbo_tasks_backend::{BackendOptions, TurboTasksBackend, noop_backing_storage};

#[turbo_tasks::value(transparent)]
#[derive(Debug)]
struct FibResult(u64);

#[turbo_tasks::function]
async fn fib(i: u32, key: u32) -> anyhow::Result<Vc<FibResult>> {
    Ok(match i {
        0 => FibResult(1).cell(),
        1 => fib(0, key),
        _ => {
            let a = fib(i - 1, key);
            let b = fib(i - 2, key);
            FibResult(a.await?.wrapping_add(*b.await?)).cell()
        }
    })
}
pub async fn main_inner() -> anyhow::Result<()> {
    let tt = TurboTasks::new(TurboTasksBackend::new(
        BackendOptions::default(),
        noop_backing_storage(),
    ));

    let task = tt.spawn_root_task(|| async {
        let output = fib(10, 0).await?;
        dbg!(output);
        Ok::<Vc<()>, _>(Default::default())
    });
    tt.wait_task_completion(task, turbo_tasks::ReadConsistency::Strong)
        .await?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let rt =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(main_inner())?;
    Ok((    ))
}