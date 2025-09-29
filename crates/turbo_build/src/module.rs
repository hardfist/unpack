use turbo_tasks::{ResolvedVc, Vc};

use crate::asset_content::AssetContent;


#[turbo_tasks::value]
#[derive(Clone, Debug)]
pub struct Module {
    pub content: ResolvedVc<AssetContent>,
}

#[turbo_tasks::value_impl]
impl Module {
    #[turbo_tasks::function]
    pub fn new(content: ResolvedVc<AssetContent>) -> Vc<Module> {
        Module {
            content
        }.cell()
    }
}