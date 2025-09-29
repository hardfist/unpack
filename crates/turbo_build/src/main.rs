#![feature(arbitrary_self_types_pointers)]
use std::{env::current_dir, path::PathBuf};
use anyhow::{Result,Ok};
use turbo_tasks_fs::{DiskFileSystem, FileContent, FileSystem, FileSystemEntryType, FileSystemPath};
use turbo_tasks::{ResolvedVc, TaskInput, TurboTasks, Vc};
use turbo_tasks_backend::{BackendOptions, TurboTasksBackend, noop_backing_storage};
#[turbo_tasks::value]
#[derive(Clone, Debug, Hash)]
struct FileSource {
    path: FileSystemPath
}
#[turbo_tasks::value_impl]
impl FileSource {
    #[turbo_tasks::function]
    async fn content(&self) -> anyhow::Result<Vc<AssetContent>> {
        let file_type = &*self.path.get_type().await?;
        match file_type {
            FileSystemEntryType::File => {
                Ok(AssetContent::File(self.path.read().to_resolved().await?).cell())
            }
            FileSystemEntryType::NotFound => {
                Ok(AssetContent::File(FileContent::NotFound.resolved_cell()).cell())
            }
            _ => Err(anyhow::anyhow!("Invalid file type {:?}", file_type)),
        }
    }
}

#[turbo_tasks::value]
struct AssetContent {
    content: ResolvedVc<FileContent>,
}
impl AssetContent {
    fn File(content: ResolvedVc<FileContent>) -> Self {
        AssetContent { content }
    }
}
#[turbo_tasks::value]
#[derive(Clone, Debug)]
struct Module {
    content: ResolvedVc<AssetContent>
}
#[turbo_tasks::function]
async fn parse(source_file: ResolvedVc<FileSource>) -> Result<Vc<Module>> {
    let content = source_file.content().to_resolved().await?;
    let module = Module { content };
    Ok(module.cell())
}
#[turbo_tasks::function]
async fn bundle(entry: Vc<FileSource>) -> anyhow::Result<Vc<()>> {
   let entry = entry.to_resolved().await?;
   let result = parse(*entry).await?;
   Ok(Vc::cell(()))
}
pub async fn main_inner() -> anyhow::Result<()> {
    
    

    let tt = TurboTasks::new(TurboTasksBackend::new(
        BackendOptions::default(),
        noop_backing_storage(),
    ));

    let task = tt.spawn_root_task(|| async {
        let root = current_dir().unwrap().join("/fixtures").to_str().unwrap().to_string();
        let fs = DiskFileSystem::new("disk_fs".into(), root.into());
        let entry_path = fs.root().await?.join("input")?;
        let entry_module = FileSource { path: entry_path }.cell();
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