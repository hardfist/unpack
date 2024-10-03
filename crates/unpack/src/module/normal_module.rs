use camino::Utf8PathBuf;
use rspack_sources::BoxSource;
use crate::errors::Diagnostics;
use crate::errors::miette::Result;

use super::{BuildContext, BuildResult, Module, ModuleIdentifier};
#[derive(Debug)]
pub struct NormalModule {
    id: ModuleIdentifier,
    resource_path: Utf8PathBuf,
    request:String,
    diagnostics: Diagnostics,
    original_source: Option<BoxSource>,
}
impl Module for NormalModule {
    fn build(&mut self,build_context: BuildContext) -> Result<BuildResult> {
        Ok(BuildResult{})
    }
}
impl NormalModule {
    pub(crate) fn new(request: String,resource_path: Utf8PathBuf) -> Self {
        let id = Self::gen_id(&request);
        Self {
            id,
            request,
            resource_path,
            diagnostics: vec![],
            original_source: None,
        }
    }
    fn gen_id(request: &str) -> ModuleIdentifier {
        ModuleIdentifier(request.to_string())
    }
}
