use crate::{
    normal_module_factory::NormalModuleFactory,
    resolver_factory::ResolverFactory,
    task::Task
};
use crate::task::{AddTask, BuildTask, FactorizeTask, ProcessDepsTask, TaskQueue};
use camino::Utf8PathBuf;
use std::collections::VecDeque;
use std::sync::Arc;

use crate::{
    compiler::CompilerOptions,
    dependency::{DependencyId, EntryDependency},
    module_graph::ModuleGraph,
};
pub(crate) struct ModuleScanner {
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
        task_queue: &mut VecDeque<Task>,
        dependencies: Vec<DependencyId>,
    ) {
        dependencies
            .iter()
            .filter_map(|id| {
                let dep = id.get_dependency(module_graph).clone();
                dep.into_module_dependency().map(|mod_dependency| {
                    (id, mod_dependency)
                })
            })
            .for_each(|(_id, dep)| {
                task_queue.push_back(Task::Factorize(FactorizeTask{
                    module_dependency: dep,
                    origin_module_id: None,
                    options: self.options.clone(),
                    module_factory: Arc::new(NormalModuleFactory {
                        options: self.options.clone(),
                        context: self.options.context.clone(),
                        resolver_factory: self.resolver_factory.clone(),
                    }),
                }));
            });
    }
    pub fn resolve_module() {}
}

/// main loop task
impl ModuleScanner {
    pub fn build_loop(&mut self, module_graph: &mut ModuleGraph, dependencies: Vec<DependencyId>) {
        let mut task_queue = TaskQueue::new();
        // kick off entry dependencies to task_queue
        self.handle_module_creation(module_graph, &mut task_queue, dependencies);
        while let Some(task) = task_queue.pop_front() {
            self.handle_task(task);
        }
    }
    fn handle_task(&mut self, task: Task){
        match task {
            Task::Factorize(factorize_task) => {
                self.handle_factorize(factorize_task);
            },
            Task::Add(add_task) => {
                self.handle_add(add_task);
            },
            Task::Build(task) => {
                self.handle_build(task);
            },
            Task::ProcessDeps(task) => {
                self.handle_process_deps(task);
            }
        }
    }
    fn handle_factorize(&self,_task: FactorizeTask) {}
    fn handle_process_deps(&self,_task: ProcessDepsTask) {}
    fn handle_add(&self, _task: AddTask) {

    }
    fn handle_build(&self, _task: BuildTask) {

    }
}
