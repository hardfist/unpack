use std::{path::PathBuf, sync::Arc};
use derive_new::new;
use crate::{module::NormalModule, dependency::{BoxDependency, Dependency, EntryDependency}, compiler::CompilerOptions};

#[derive(new)]
pub struct Compilation {
    #[allow(dead_code)]
    options: Arc<CompilerOptions>
}

impl Compilation {
    /// similar with webpack's make phase, which will make module graph
    pub fn scan(&mut self){
        println!("start scan");
        
    }
    /// similar with webpack's seal phase
    /// this will make chunk(consists of connected modules)
    pub fn link(&mut self){
        println!("start link")
    }

    // build module graph
    pub fn build_module_graph(){

    }
    // add_entry
    pub fn add_entry(&mut self){
        let entry_dep = EntryDependency::new(
            self.options.entry.import.clone(),
            self.options.context.clone()
        );
        self.add_module_tree(self.options.context.clone(), entry_dep)
    }
    // add_module_tree
    pub fn add_module_tree(&mut self,context: PathBuf, dependency: EntryDependency){

        self.handle_module_creation(vec![Box::new(dependency)],None);
    }
    // handle_module_creation
    pub fn handle_module_creation(&mut self,dependencies: Vec<BoxDependency>, origin_module: Option<NormalModule>){

    }
    // factorize_module
    pub fn factorize_module(){

    }
    // add_module
    pub fn add_module(){

    }
    pub fn handle_module_build_and_dependencies(){

    }


}