use crate::compiler::CompilerOptions;
use camino::Utf8PathBuf;
use miette::Result;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
pub struct PluginContext {
    pub options: Arc<CompilerOptions>,
}
pub struct LoadArgs {
    pub path: Utf8PathBuf,
    pub context: Utf8PathBuf
}

pub trait Plugin: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn load(&self, _ctx: PluginContext, _args: LoadArgs) -> Result<Option<String>> {
        Ok(None)
    }
}

pub type BoxPlugin = Arc<dyn Plugin>;

#[derive(Clone,Debug)]
pub struct PluginDriver {
    pub plugins: Vec<BoxPlugin>
}