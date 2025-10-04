use turbo_tasks::{ResolvedVc, Vc};
use turbo_tasks_fs::FileContent;

use crate::asset::Asset;


#[turbo_tasks::value]
#[derive(Debug)]
pub struct AssetContent {
    pub content: ResolvedVc<FileContent>,
}
impl AssetContent {
    pub fn new(content: ResolvedVc<FileContent>) -> Vc<Self> {
        Self { content }.cell()
    }
}
