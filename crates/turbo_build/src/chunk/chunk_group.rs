use turbo_tasks::{trace::TraceRawVcs, NonLocalValue, ResolvedVc, TaskInput};

use crate::module::Module;

#[derive(Debug, Clone,TaskInput,TraceRawVcs,Hash,PartialEq,Eq,serde::Serialize,serde::Deserialize,NonLocalValue)]
pub enum ChunkGroupEntry {
    Entry(Vec<ResolvedVc<Box<dyn Module>>>),
    Async(ResolvedVc<Box<dyn Module>>),
}