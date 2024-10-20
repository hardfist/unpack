use crate::compiler::CompilerOptions;
use camino::Utf8PathBuf;
use miette::Result;
use std::sync::Arc;

#[derive(Clone)]
pub struct PluginContext {
    pub options: Arc<CompilerOptions>,
}
pub struct LoadArgs {
    pub path: Utf8PathBuf,
}

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn load(&self, _ctx: PluginContext, _args: LoadArgs) -> Result<Option<String>> {
        Ok(None)
    }
}
