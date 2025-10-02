use turbo_rcstr::{rcstr, RcStr};
use turbo_tasks::{ResolvedVc, TaskInput, Vc};

use crate::{chunk::{chunk_context::{ChunkGroupResult, ChunkingContext}, chunk_group::ChunkGroup}, ident::AssetIdent, module_graph::ModuleGraph};

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
