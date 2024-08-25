use std::{collections::VecDeque, sync::Arc};
use crate::task_queue::{AddTask,BuildTask,FactorizeTask,ProcessDepsTask, Task};
use derive_new::new;

use crate::{compiler::CompilerOptions, dependency::{BoxDependency, DependencyId, EntryDependency}, module::{Module, ModuleId, NormalModule}, module_graph::ModuleGraph, task_queue::TaskQueue};

#[derive(new)]
pub struct ModuleScanner {
    options: Arc<CompilerOptions>
}
struct FactorizeParams {

}
impl ModuleScanner {
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
            match task {
                Task::Factorize(FactorizeTask { module_dependency, origin_module_id }) => {
                    dbg!(module_dependency, origin_module_id);
                },
                _ => {

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
        dbg!(dep.as_module_dependency());
        // only deal with module_dependency
        if let Some(mod_dependency) = dep.as_module_dependency() {
            Some((id, mod_dependency))
        }else {
            None
        }
    }).for_each(|(id, dep)| {
        task_queue.add_task(Task::Factorize(FactorizeTask {
            module_dependency: *id,
            origin_module_id: None
        }));
    });
   }
   pub fn resolve_module(){

   }
}
