use turbo_tasks::{ResolvedVc, Vc};
 use crate::chunk::chunk_context::ChunkingContext;
use crate::{chunk::chunk_item::ChunkItem, ident::AssetIdent, output::OutputAssets};

#[turbo_tasks::value_trait]
pub trait Chunk {
    #[turbo_tasks::function]
    fn ident(self: Vc<Self>) -> Vc<AssetIdent>;
    #[turbo_tasks::function]
    fn chunking_context(self: Vc<Self>) -> Vc<Box<dyn ChunkingContext>>;
    // fn path(self: Vc<Self>) -> Vc<FileSystemPath> {
    //     self.ident().path()
    // }

    /// Other [OutputAsset]s referenced from this [Chunk].
    #[turbo_tasks::function]
    fn references(self: Vc<Self>) -> Vc<OutputAssets> {
        OutputAssets::empty()
    }

    #[turbo_tasks::function]
    fn chunk_items(self: Vc<Self>) -> Vc<ChunkItems> {
        ChunkItems(vec![]).cell()
    }
}

#[turbo_tasks::value(transparent)]
pub struct ChunkItems(pub Vec<ResolvedVc<Box<dyn ChunkItem>>>);
