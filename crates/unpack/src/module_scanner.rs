use crate::{
    normal_module_factory::NormalModuleFactory,
    resolver_factory::ResolverFactory,
    task::Task
};
use crate::task::FactorizeTask;
use camino::Utf8PathBuf;
use std::sync::{mpsc::{self, Sender}, Arc};

use crate::{
    compiler::CompilerOptions,
    dependency::{DependencyId, EntryDependency},
    module_graph::ModuleGraph,
};
pub struct ModuleScanner {
    options: Arc<CompilerOptions>,
    context: Utf8PathBuf,
    resolver_factory: Arc<ResolverFactory>,
}
struct FactorizeParams {}
impl ModuleScanner {
    pub fn new(options: Arc<CompilerOptions>, context: Utf8PathBuf) -> Self {
        Self {
            options: options.clone(),
            context,
            resolver_factory: Arc::new(ResolverFactory::new_with_base_option(
                options.resolve.clone(),
            )),
            // make_artifact: Default::default(),
        }
    }
    // add_entry
    pub fn add_entry(&mut self, module_graph: &mut ModuleGraph) {
        let entry_dep = EntryDependency::new(
            self.options.entry.import.clone(),
            self.options.context.clone(),
        );
        let entry_dep_id = module_graph.add_dependency(Box::new(entry_dep));
        self.build_loop(module_graph, vec![entry_dep_id]);
    }
    pub fn handle_module_creation(
        &mut self,
        module_graph: &mut ModuleGraph,
        task_queue: &mut Sender<Task>,
        dependencies: Vec<DependencyId>,
    ) {
        dependencies
            .iter()
            .filter_map(|id| {
                let dep = id.get_dependency(module_graph);
                // only deal with module_dependency
                dep.as_module_dependency().map(|mod_dependency| (id, dep.clone()))
            })
            .for_each(|(id, dep)| {
                let dep_new = dep.clone();
                task_queue.send(Task::FactorizeTask(FactorizeTask{
                    module_dependency: dep,
                    origin_module_id: None,
                    options: self.options.clone(),
                    module_factory: Arc::new(NormalModuleFactory {
                        options: self.options.clone(),
                        context: self.options.context.clone(),
                        resolver_factory: self.resolver_factory.clone(),
                    }),
                })).expect("send failed");
            });
    }
    pub fn resolve_module() {}
}

/// main loop task
impl ModuleScanner {
    pub fn build_loop(&mut self, module_graph: &mut ModuleGraph, dependencies: Vec<DependencyId>) {
        let (mut send,recv)= mpsc::channel::<Task>();
        // kick off entry dependencies to task_queue
        self.handle_module_creation(module_graph, &mut send, dependencies);
        while let Ok(task) = recv.recv() {
            self.handle_task(task);
        }
    }
    fn handle_task(&mut self, task: Task){

    }
    fn factorize() {}
    fn process_dependencies() {}
}
