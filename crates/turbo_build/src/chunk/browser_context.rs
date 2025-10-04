use turbo_rcstr::{rcstr, RcStr};
use turbo_tasks::{FxIndexSet, ResolvedVc, TaskInput, Vc};
use anyhow::Result;
use crate::{chunk::{availability_info::{AvailabilityInfo, ChunkableModule, ChunkableModuleOrBatch}, chunk_context::{ChunkGroupResult, ChunkingContext}, chunk_group::ChunkGroup}, ident::AssetIdent, module::Module, module_graph::{ModuleBatchGroup, ModuleGraph}};

#[turbo_tasks::value]
#[derive(Debug, Clone, Hash, TaskInput)]
pub struct BrowserChunkingContext {
    name: Option<RcStr>,
}

impl BrowserChunkingContext {
    pub fn builder() -> BrowserChunkingContextBuilder {
        BrowserChunkingContextBuilder {
            chunking_context: BrowserChunkingContext { name: None },
        }
    }
}
pub struct BrowserChunkingContextBuilder {
    chunking_context: BrowserChunkingContext,
}
impl BrowserChunkingContextBuilder {
    pub fn name(mut self, name: RcStr) -> Self {
        self.chunking_context.name = Some(name);
        self
    }
    pub fn build(self) -> Vc<BrowserChunkingContext> {
        BrowserChunkingContext::cell(self.chunking_context)
    }
}

#[turbo_tasks::value_impl]
impl ChunkingContext for BrowserChunkingContext{
    #[turbo_tasks::function]
    fn name(&self) -> Vc<RcStr>{
         if let Some(name) = &self.name {
            Vc::cell(name.clone())
        } else {
            Vc::cell(rcstr!("unknown"))
        }
    }
    #[turbo_tasks::function]
    fn evaluated_chunk_group(self:ResolvedVc<Self> ,ident:Vc<AssetIdent> ,chunk_group:ChunkGroup,module_graph:Vc<ModuleGraph>) -> Vc<ChunkGroupResult> {
        todo!()
    }
}

pub async fn make_chunk_group(
    chunk_group_entries: impl IntoIterator<
        IntoIter = impl Iterator<Item = ResolvedVc<Box<dyn Module>>> + Send,
    > + Send
    + Clone,
    module_graph: Vc<ModuleGraph>,
    chunking_context: ResolvedVc<Box<dyn ChunkingContext>>,
    availability_info: AvailabilityInfo,
){

}
pub async fn chunk_group_content(
    module_graph: Vc<ModuleGraph>,
    chunk_group_entries: impl IntoIterator<
        IntoIter = impl Iterator<Item = ResolvedVc<Box<dyn Module>>> + Send,
    > + Send,
    availability_info: AvailabilityInfo,
    can_split_async: bool,
    should_trace: bool,
    should_merge_modules: bool,
    batching_config: Vc<BatchingConfig>,
) -> Result<ChunkGroupContent>{
    
    todo!()
}

#[turbo_tasks::value]
#[derive(Debug, Clone, Default, TaskInput, Hash)]
pub struct BatchingConfig {
    /// Use a heuristic based on the module path to create batches. It aims for batches of a good
    /// size.
    pub use_heuristic: bool,
}

pub struct ChunkGroupContent {
    pub chunkable_items: Vec<ChunkableModuleOrBatch>,
    pub batch_groups: Vec<ResolvedVc<ModuleBatchGroup>>,
    pub async_modules: FxIndexSet<ResolvedVc<Box<dyn ChunkableModule>>>,
    pub traced_modules: FxIndexSet<ResolvedVc<Box<dyn Module>>>,
    pub availability_info: AvailabilityInfo,
}
