use turbo_tasks::{ResolvedVc, Vc};
use turbo_tasks_fs::FileContent;


#[turbo_tasks::value]
#[derive(Debug)]
pub struct AssetContent {
    pub content: ResolvedVc<FileContent>,
}
impl AssetContent {
    pub fn file(content: ResolvedVc<FileContent>) -> Vc<Self> {
        Self { content }.cell()
    }
}
