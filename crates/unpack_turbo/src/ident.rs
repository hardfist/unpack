use turbo_tasks::{TaskInput, Vc};
use turbo_tasks_fs::{ FileSystemPath};

#[turbo_tasks::value]
#[derive(Clone,Debug,Hash,TaskInput)]
pub struct AssetIdent {
    pub path: FileSystemPath 
}

#[turbo_tasks::value_impl]
impl AssetIdent {
    #[turbo_tasks::function]
    pub fn from_path(path: FileSystemPath) -> Vc<AssetIdent> {
        AssetIdent { path: path.clone() }.cell()
    }
    #[turbo_tasks::function]
    pub fn path(&self) -> Vc<FileSystemPath>{
        self.path.clone().cell()
    }

}