use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use turbo_rcstr::RcStr;
use turbo_tasks::{ trace::TraceRawVcs, NonLocalValue, TaskInput, TryJoinIterExt, TurboTasks, Vc};
use turbo_tasks_backend::{NoopBackingStorage, TurboTasksBackend};
use turbo_tasks_fs::{DiskFileSystem, FileSystem};
use crate::resolve::{parse::Request};
#[derive(
    Clone, Debug, TaskInput, Hash, PartialEq, Eq, NonLocalValue, Serialize, Deserialize, TraceRawVcs,
)]
pub enum EntryRequest {
    Relative(RcStr),
    Module(RcStr, RcStr),
}
type Backend = TurboTasksBackend<NoopBackingStorage>;
pub struct UnpackBuilder {
    turbo_tasks: Arc< TurboTasks<Backend>>,
    project_dir: RcStr,
    entry_requests: Vec<EntryRequest>
}

impl UnpackBuilder {
    pub fn new(turbo_tasks: Arc<TurboTasks<Backend>>,project_dir: RcStr) -> Self {
        Self {
            turbo_tasks,
            project_dir,
            entry_requests: Vec::new(),
        }
    }

    pub fn add_entry_request(&mut self, entry_request: EntryRequest) {
        self.entry_requests.push(entry_request);
    }

    pub async fn build(self) -> Result<(), anyhow::Error> {

        let task = self.turbo_tasks.spawn_once_task::<(),_>(async move {
            let build_result_op = build_internal(self.project_dir.clone(), self.entry_requests.clone());
            Ok(Default::default())
        });
        Ok(())
    }
}

#[turbo_tasks::function(operation)]
async fn build_internal(
    project_dir: RcStr,
    entry_requests: Vec<EntryRequest>,
) -> anyhow::Result<()>
{
    let output_fs = output_fs(project_dir.clone());
    let build_output_root = output_fs.root().await?.join("dist")?;
    let entry_requests = (*entry_requests.into_iter().map(|r| async move {
         Ok(match r {
                EntryRequest::Relative(p) => Request::relative(
                    p.clone().into(),
                    Default::default(),
                    Default::default(),
                    false,
                ),
                EntryRequest::Module(m, p) => Request::module(
                    m.clone(),
                    p.clone().into(),
                    Default::default(),
                    Default::default(),
                ),
            })
    })
    .try_join()
    .await?).to_vec();

    Ok(Default::default())
}

#[turbo_tasks::function]
pub fn output_fs(project_dir: RcStr) -> Result<Vc<Box<dyn FileSystem>>> {
    let disk_fs = DiskFileSystem::new("output".into(),project_dir.clone(), vec![]);
    Ok(Vc::upcast(disk_fs))
    
}

