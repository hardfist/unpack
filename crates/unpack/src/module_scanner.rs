use std::{collections::VecDeque, option, sync::Arc};
use crate::{dependency::{BoxModuleDependency, ModuleDependency}, normal_module_factory::NormalModuleFactory, resolver_factory::ResolverFactory, task::{AddTask,BuildTask,FactorizeTask,ProcessDepsTask, Task}};
use camino::Utf8PathBuf;
use derive_new::new;

use crate::{compiler::CompilerOptions, dependency::{BoxDependency, DependencyId, EntryDependency}, module::{Module, ModuleId, NormalModule}, module_graph::ModuleGraph, task::TaskQueue};


pub struct ModuleScanner {
    options: Arc<CompilerOptions>,
    context: Utf8PathBuf,
    resolver_factory: Arc<ResolverFactory>,
    errors: Vec<Box<dyn miette::Diagnostic + Send + Sync>>
}
struct FactorizeParams {

}
impl ModuleScanner {
    pub fn new(options: Arc<CompilerOptions>, context: Utf8PathBuf) -> Self{
        Self {
            options: options.clone(), 
            context,
            resolver_factory: Arc::new(ResolverFactory::new_with_base_option(options.resolve.clone())),
            errors: vec![]
        }
    }
    // add_entry
    pub fn add_entry(&mut self, module_graph: &mut ModuleGraph) {
        let entry_dep = EntryDependency::new(
            self.options.entry.import.clone(),
            self.options.context.clone(),
        );
        let entry_dep_id = module_graph.add_dependency(Box::new(entry_dep));
        self.build_loop(module_graph,vec![entry_dep_id]);
    }
   pub fn build_loop(&mut self,module_graph: &mut ModuleGraph, dependencies: Vec<DependencyId>){
        let mut task_queue = TaskQueue::new();
        self.handle_module_creation(module_graph,&mut task_queue, dependencies);
        
        while let Some(task) = task_queue.get_next_task() {
            match task.run() {
                Ok(new_task) => {
                    task_queue.add_tasks(new_task);
                },
                Err(err) => {
                    self.errors.push(err.into());
                }
            }        
        }
   }
pub fn handle_module_creation(
    &mut self,
    module_graph: &mut ModuleGraph,
    task_queue: &mut TaskQueue,
    dependencies: Vec<DependencyId>
){
    dependencies.iter().filter_map(|id| {
        let dep =  id.get_dependency(&module_graph);
        // only deal with module_dependency
        if let Some(mod_dependency) = dep.as_module_dependency() {
            Some((id, dep.clone()))
        }else {
            None
        }
    }).for_each(|(id, dep)| {
        let dep_new = dep.clone();
        task_queue.add_task(Box::new(FactorizeTask {
            module_dependency: dep,
            origin_module_id: None,
            options: self.options.clone(),
            module_factory: Arc::new(NormalModuleFactory{
                options: self.options.clone(),
                context: self.options.context.clone(),
                resolver_factory: self.resolver_factory.clone()
            })
        }));
    });
}
   pub fn resolve_module(){

   }
}
