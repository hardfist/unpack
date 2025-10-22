
use std::path::Path;
use unpack_turbo::{asset::Asset, chunk::chunk_group::ChunkGroupEntry, file_source::FileSource, module::{EcmascriptModuleAsset, Module}, module_graph::ModuleGraph};
use anyhow::Ok;
use turbo_tasks_fs::{DiskFileSystem, FileSystem};
use turbo_tasks::{vdbg, ResolvedVc, TurboTasks, Vc};
use turbo_tasks_backend::{BackendOptions, TurboTasksBackend, noop_backing_storage};

#[turbo_tasks::function]
async fn bundle(entry: Vc<FileSource>) -> anyhow::Result<Vc<()>> {
   let entry = entry.to_resolved().await?;
   let module = EcmascriptModuleAsset::new(*ResolvedVc::upcast(entry)).to_resolved().await?;
   let module = ResolvedVc::upcast::<Box<dyn Module>>(module);
   let graph_entries = Vc::cell(vec![ChunkGroupEntry::Entry(vec![module])]);
   let module_graph = ModuleGraph::from_entries(graph_entries);
//    let browser_context = BrowserChunkingContext::builder().name("main".into()).build();
//    let ident = entry.ident();
//    let chunk_group = ChunkGroup::Entry(vec![module]);
//    let result = browser_context.evaluated_chunk_group_assets( ident, chunk_group, module_graph);
   let cg = module_graph.chunk_group_info().await?;
   vdbg!(cg);
   Ok(Vc::cell(()))
}
pub async fn main_inner() -> anyhow::Result<()> {
    
    

    let tt = TurboTasks::new(TurboTasksBackend::new(
        BackendOptions::default(),
        noop_backing_storage(),
    ));

    let task = tt.spawn_root_task(|| async {

        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("./fixtures").canonicalize().unwrap().to_str().unwrap().to_string();
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