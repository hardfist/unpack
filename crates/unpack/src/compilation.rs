use crate::{
    compiler::CompilerOptions, errors::Diagnostics, module_graph::ModuleGraph, module_scanner::ModuleScanner
};
use std::sync::Arc;

pub struct Compilation {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>,
    module_graph: ModuleGraph,
    pub(crate) diagnostics: Diagnostics,
}

impl Compilation {
    pub fn new(options: Arc<CompilerOptions>) -> Self {
        Self {
            options,
            module_graph: Default::default(),
            diagnostics: Default::default(),
        }
    }
    /// similar with webpack's make phase, which will make module graph
    pub fn scan(&mut self) {
        println!("start scan");
        let mut module_scanner =
            ModuleScanner::new(self.options.clone(), self.options.context.clone());
        module_scanner.add_entry(&mut self.module_graph);
        // self.diagnostics
        //     .extend(module_scanner.make_artifact.diagnostics);
    }
    /// similar with webpack's seal phase
    /// this will make chunk(consists of connected modules)
    pub fn link(&mut self) {
        println!("start link")
    }

    // build module graph
    pub fn build_module_graph() {}
}
