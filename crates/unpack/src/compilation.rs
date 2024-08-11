use crate::{
    compiler::CompilerOptions,
    dependency::{BoxDependency, Dependency, EntryDependency},
    module::NormalModule,
};
use derive_new::new;
use std::{path::PathBuf, sync::Arc};

#[derive(new)]
pub struct Compilation {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>,
}

impl Compilation {
    /// similar with webpack's make phase, which will make module graph
    pub fn scan(&mut self) {
        println!("start scan");
    }
    /// similar with webpack's seal phase
    /// this will make chunk(consists of connected modules)
    pub fn link(&mut self) {
        println!("start link")
    }

    // build module graph
    pub fn build_module_graph() {}
}
