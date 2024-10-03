use camino::Utf8PathBuf;
use miette::IntoDiagnostic;
use rspack_sources::BoxSource;
use crate::errors::Diagnostics;
use crate::errors::miette::Result;

use super::{ast::parse, BuildContext, BuildResult, Module, ModuleIdentifier};
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
        let content = std::fs::read_to_string(&self.resource_path).into_diagnostic()?;
        let ast = parse(content)?;
        Ok(BuildResult{
            ast
        })
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
    pub(crate) fn parse(){
        

    }
    fn gen_id(request: &str) -> ModuleIdentifier {
        ModuleIdentifier(request.to_string())
    }
}
