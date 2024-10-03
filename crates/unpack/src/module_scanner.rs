use crate::errors::Diagnostics;
use crate::module::BuildContext;
use crate::normal_module_factory::{ModuleFactoryCreateData, NormalModuleFactory};
use crate::{
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
    module_factory: Arc<NormalModuleFactory>,
    diagnostics: Diagnostics,
    scanner_state: ScannerState
}
struct FactorizeParams {}
impl ModuleScanner {
    pub fn new(options: Arc<CompilerOptions>, context: Utf8PathBuf) -> Self {
        let resolver_factory = Arc::new(ResolverFactory::new_with_base_option(
                options.resolve.clone(),
            ));
        let module_factory = Arc::new(NormalModuleFactory{
                options: options.clone(),
                resolver_factory: resolver_factory.clone(),
                context: context.clone()
            });
        Self {
            options: options.clone(),
            context,
            resolver_factory: resolver_factory.clone(),
            module_factory,
            diagnostics: vec![],
            scanner_state: ScannerState::default()
            // make_artifact: Default::default(),
        }
    }
    // add_entry
    pub fn add_entry(&mut self, state: &mut ScannerState) {
        let entry_dep = EntryDependency::new(
            self.options.entry.import.clone(),
            self.options.context.clone(),
        );
        let entry_dep_id = state.module_graph.add_dependency(Box::new(entry_dep));
        self.build_loop( state,vec![entry_dep_id]);
    }
    pub fn handle_module_creation(
        &self,
        state: &mut ScannerState,
        dependencies: Vec<DependencyId>,
    ) {
        dependencies
            .iter()
            .filter_map(|id| {
                let dep = id.get_dependency(&state.module_graph).clone();
                dep.into_module_dependency().map(|mod_dependency| {
                    (id, mod_dependency)
                })
            })
            .for_each(|(_id, dep)| {
                
                state.task_queue.push_back(Task::Factorize(FactorizeTask{
                    module_dependency: dep,
                    origin_module_id: None,
                }));
            });
    }
    pub fn resolve_module() {}
}

#[derive(Debug,Default)]
pub(crate) struct ScannerState {
    module_graph: ModuleGraph,
    task_queue: VecDeque<Task>,
    diagnostics: Diagnostics
}
/// main loop task
impl ModuleScanner {
    pub fn build_loop(&self,state: &mut ScannerState, dependencies: Vec<DependencyId>) {
        // kick off entry dependencies to task_queue
        self.handle_module_creation(state,dependencies);
        while let Some(task) = state.task_queue.pop_front() {
            
            self.handle_task(task, state);
        }
    }
    fn handle_task(&self, task: Task,state:&mut ScannerState){
        match task {
            Task::Factorize(factorize_task) => {
                self.handle_factorize(state,factorize_task);
            },
            Task::Add(add_task) => {
                self.handle_add(state,add_task);
            },
            Task::Build(task) => {
                self.handle_build(state,task);
            },
            Task::ProcessDeps(task) => {
                self.handle_process_deps(state,task);
            }
        }
    }
    fn handle_factorize(&self,state: &mut ScannerState,task: FactorizeTask) {
        match self.module_factory.create(ModuleFactoryCreateData{
            module_dependency: task.module_dependency,
            context: self.options.context.clone(),
            options: self.options.clone()
        }) {
            Ok(factory_result) => {
               let module_id = state.module_graph.add_module(Box::new(factory_result.module));
               state.task_queue.push_back(Task::Add(AddTask{
                 module_id
               }));
            },
            Err(err) => {
                state.diagnostics.push(err);
            }
        }
        
    }
    fn handle_process_deps(&self,state: &mut ScannerState,_task: ProcessDepsTask) {}
    fn handle_add(&self,state: &mut ScannerState, task: AddTask) {
        // do nothing here, cause we don't need module_graph_module here
        state.task_queue.push_back(Task::Build(BuildTask {
            module_id: task.module_id
        }));
    }
    fn handle_build(&self,state: &mut ScannerState, task: BuildTask) {
        let module = state.module_graph.module_by_id_mut(task.module_id);
        let build_result = module.build(BuildContext{
            options: self.options.clone()
        });
    }
}
