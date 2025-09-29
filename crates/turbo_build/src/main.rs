
use std::env::current_dir;
use turbo_build::{asset::Asset, file_source::FileSource, module::EcmascriptModule};
use turbo_tasks::vdbg;
use anyhow::{Result,Ok};
use turbo_tasks_fs::{DiskFileSystem, FileSystem};
use turbo_tasks::{ResolvedVc, TurboTasks, Vc};
use turbo_tasks_backend::{BackendOptions, TurboTasksBackend, noop_backing_storage};

#[turbo_tasks::function]
async fn parse(source_file: ResolvedVc<FileSource>) -> Result<Vc<EcmascriptModule>> {
    let module = EcmascriptModule::new(*ResolvedVc::upcast(source_file));
    Ok(module)
}
#[turbo_tasks::function]
async fn bundle(entry: Vc<FileSource>) -> anyhow::Result<Vc<()>> {
   let entry = entry.to_resolved().await?;
   vdbg!(entry.content().await?);
   let result = parse(*entry).await?;
   Ok(Vc::cell(()))
}
pub async fn main_inner() -> anyhow::Result<()> {
    
    

    let tt = TurboTasks::new(TurboTasksBackend::new(
        BackendOptions::default(),
        noop_backing_storage(),
    ));

    let task = tt.spawn_root_task(|| async {
        let root = current_dir().unwrap().join("./fixtures").canonicalize().unwrap().to_str().unwrap().to_string();
        let fs = DiskFileSystem::new("disk_fs".into(), root.into());
        let entry_path = fs.root().await?.join("input")?;
        let entry_module: Vc<FileSource> = FileSource::new(entry_path.clone());
        let content= entry_module.content().await?.content;
        let output = bundle(entry_module).await?;
        Ok::<Vc<()>>(Default::default())
    });
    tt.wait_task_completion(task, turbo_tasks::ReadConsistency::Strong)
        .await?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let rt =  tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(main_inner())?;
    Ok(())
}