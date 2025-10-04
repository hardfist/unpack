use turbo_tasks::{util::Chunk, ResolvedVc, Vc};

use crate::module::{EcmascriptModuleAsset, Module};

#[turbo_tasks::value]
struct ModuleChunkItem{
    module: ResolvedVc<EcmascriptModuleAsset>
}
#[turbo_tasks::value_trait]
pub trait ChunkItem{
    #[turbo_tasks::function]
    fn module(self: Vc<Self>) -> Vc<Box<dyn Module>>;
}
#[turbo_tasks::value_impl]
impl ChunkItem for ModuleChunkItem {
    #[turbo_tasks::function]
    fn module(&self) -> Vc<Box<dyn Module>> {
        
        *ResolvedVc::upcast(self.module)
    }
}