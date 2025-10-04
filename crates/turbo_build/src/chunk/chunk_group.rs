
use turbo_tasks::{trace::TraceRawVcs, NonLocalValue, ResolvedVc, TaskInput, Vc};
use crate::chunk::availability_info::AvailabilityInfo;
use crate::output::OutputAsset;
 use crate::chunk::chunk::Chunk;
use crate::{chunk::chunk_context::ChunkingContext, module::Module, module_graph::ModuleGraph};
use anyhow::Result;
#[derive(Debug, Clone,TaskInput,TraceRawVcs,Hash,PartialEq,Eq,serde::Serialize,serde::Deserialize,NonLocalValue)]
pub enum ChunkGroupEntry {
    Entry(Vec<ResolvedVc<Box<dyn Module>>>),
    Async(ResolvedVc<Box<dyn Module>>),
}
impl ChunkGroupEntry {
    pub fn entries(&self) -> Vec<ResolvedVc<Box<dyn Module>>> {
        match self {
            ChunkGroupEntry::Entry(modules) => modules.clone(),
            ChunkGroupEntry::Async(module) => vec![module.clone()],
        }
    }
}

#[derive(Debug, Clone,TaskInput,TraceRawVcs,Hash,PartialEq,Eq,serde::Serialize,serde::Deserialize,NonLocalValue)]
pub enum ChunkGroup {
    Entry(Vec<ResolvedVc<Box<dyn Module>>>),
    Async(ResolvedVc<Box<dyn Module>>),
}

/// Creates a chunk group from a set of entries.
pub async fn make_chunk_group(
    chunk_group_entries: impl IntoIterator<
        IntoIter = impl Iterator<Item = ResolvedVc<Box<dyn Module>>> + Send,
    > + Send
    + Clone,
    module_graph: Vc<ModuleGraph>,
    chunking_context: ResolvedVc<Box<dyn ChunkingContext>>,
    availability_info: AvailabilityInfo,
) -> Result<MakeChunkGroupResult> {
    todo!()
}

pub struct MakeChunkGroupResult {
    pub chunks: Vec<ResolvedVc<Box<dyn Chunk>>>,
    pub referenced_output_assets: Vec<ResolvedVc<Box<dyn OutputAsset>>>,
    pub availability_info: AvailabilityInfo,
}
