use crate::dependency::{BlockId, BoxDependency, DependenciesBlock, DependencyId, HarmonyImportSideEffectDependency};
use crate::errors::miette::Result;
use crate::errors::Diagnostics;
use camino::{Utf8Path, Utf8PathBuf};
use miette::IntoDiagnostic;
use rspack_sources::BoxSource;
use swc_core::ecma::utils::swc_ecma_ast;

use super::{ast::parse, BuildContext, BuildResult, Module};
#[derive(Debug)]
pub struct NormalModule {
    context: Option<Utf8PathBuf>,
    resource_path: Utf8PathBuf,
    request: String,
    diagnostics: Diagnostics,
    original_source: Option<BoxSource>,
    dependencies: Vec<DependencyId>,
    blocks: Vec<BlockId>
}
struct ParseResult {
    dependencies: Vec<BoxDependency>,
}
impl DependenciesBlock for NormalModule {
    fn add_block_id(&mut self, block_id: BlockId) {
        self.blocks.push(block_id);
    }

    fn get_blocks(&self) -> Vec<BlockId> {
        self.blocks.clone()
    }

    fn add_dependency_id(&mut self, dependency_id: DependencyId) {
        self.dependencies.push(dependency_id);
    }

    fn get_dependencies(&self) -> Vec<DependencyId> {
        self.dependencies.clone()
    }
}
impl Module for NormalModule {
    fn build(&mut self, _build_context: BuildContext) -> Result<BuildResult> {
        let content = std::fs::read_to_string(&self.resource_path).into_diagnostic()?;
        let parse_result = self.parse(content)?;
        Ok(BuildResult {
            dependencies: parse_result.dependencies,
        })
    }
    fn get_context(&self) -> Option<&Utf8Path> {
        self.context.as_ref().map(|x| x.as_ref())
    }
}
impl NormalModule {
    pub fn new(request: String, resource_path: Utf8PathBuf) -> Self {
        let context = resource_path.parent().map(|x| x.to_owned());
        Self {
            request,
            resource_path,
            diagnostics: vec![],
            original_source: None,
            context,
            blocks:  vec![],
            dependencies: vec![]
        }
    }
    fn parse(&self, content: String) -> Result<ParseResult> {
        let ast = parse(content)?;
        // Analyze the AST for all import dependencies
        let mut requests = Vec::new();

        for item in &ast.program.as_module().unwrap().body {
            if let swc_ecma_ast::ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::Import(import)) =
                item
            {
                let request = import.src.value.clone();
                requests.push(request);
            }
        }
        let dependencies = requests
            .into_iter()
            .map(|request| Box::new(HarmonyImportSideEffectDependency { request }) as BoxDependency)
            .collect::<Vec<_>>();
        Ok(ParseResult { dependencies })
    }
}
