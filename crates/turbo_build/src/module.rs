use turbo_tasks::{ResolvedVc, Vc};

use crate::{asset::Asset, asset_content::AssetContent, ident::AssetIdent, reference::ModuleReferences, source::Source};

#[turbo_tasks::value_trait]
pub trait Module: Asset {
    #[turbo_tasks::function]
    fn ident(&self) -> Vc<AssetIdent>;
    #[turbo_tasks::function]
    fn references(self: Vc<Self>) -> Vc<ModuleReferences>;
}

#[turbo_tasks::value]
#[derive(Clone, Debug)]
pub struct EcmascriptModule {
    pub source: ResolvedVc<Box<dyn Source>>,
}

#[turbo_tasks::value_impl]
impl EcmascriptModule {
    #[turbo_tasks::function]
    pub fn new(source: ResolvedVc<Box<dyn Source>>) -> Vc<EcmascriptModule> {
        EcmascriptModule {
            source
        }.cell()
    }
}
impl Module for EcmascriptModule {
    fn ident(self:turbo_tasks::Vc<Self>) -> Vc<AssetIdent>where Self:Sized {
        todo!()
    }

    fn references(self:turbo_tasks::Vc<Self>) -> Vc<ModuleReferences>where Self:Sized {
        todo!()
    }
}
#[turbo_tasks::value_impl]
impl Asset for EcmascriptModule {
    #[turbo_tasks::function]
    fn content(&self) -> Vc<AssetContent>{
        self.source.content()
    }
}