use serde::{Deserialize, Serialize};
use turbo_tasks::{debug::ValueDebugFormat, trace::TraceRawVcs, NonLocalValue, ValueToString, Vc};

use crate::reference::ModuleReference;

#[derive(Debug, Clone, Hash, PartialEq, Eq,NonLocalValue, TraceRawVcs, Serialize,Deserialize, ValueDebugFormat)]
pub enum ChunkingType {
    Async,
    Parallel,
}

#[turbo_tasks::value(transparent)]
pub struct ChunkingTypeOption(Option<ChunkingType>);
#[turbo_tasks::value_trait]
pub trait ChunkableModuleReference: ModuleReference + ValueToString{
    #[turbo_tasks::function]
    fn chunking_type(self: Vec<Self>) -> Vc<ChunkingTypeOption>;
}