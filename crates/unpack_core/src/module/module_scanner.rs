use crate::dependency::{BoxDependency, DependencyId};
use crate::errors::miette::{Report, Result};
use crate::errors::Diagnostics;
use crate::memory_manager::MemoryManager;
use crate::module::{BuildContext, Module, ModuleId, RwCell, WritableModule};
use crate::normal_module_factory::{ModuleFactoryCreateData, NormalModuleFactory};
use crate::plugin::PluginDriver;
use crate::scheduler::COMPILER_CONTEXT;
use crate::task::{AddModuleTask, BuildTask, FactorizeTask};
use crate::{resolver_factory::ResolverFactory, task::Task};
use camino::Utf8PathBuf;
use dashmap::DashMap;
use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::fmt::Write;
use std::mem;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use swc_core::ecma::utils::RefRewriter;
use tokio::task::JoinSet;
use tracing::instrument;
use ustr::Ustr;

use super::module_graph::ModuleGraph;
use crate::{compiler::CompilerOptions, dependency::EntryDependency};
use tokio::sync::mpsc::{
    unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
#[derive(Debug)]
pub struct EntryData {
    name: Option<String>,
    pub(crate) dependencies: Vec<DependencyId>,
}
#[derive(Debug)]
pub struct ModuleScanner {
    options: Arc<CompilerOptions>,
    context: Utf8PathBuf,
    resolver_factory: Arc<ResolverFactory>,
    module_factory: Arc<NormalModuleFactory>,
    plugin_driver: Arc<PluginDriver>,
    working_tasks: JoinSet<()>,
    todo_tx: Sender<Result<Task>>,
    todo_rx: Receiver<Result<Task>>,
    dependency_cache: Arc<DashMap<DependencyId, WritableModule>>,
}
struct FactorizeParams {}
impl ModuleScanner {
    pub fn new(
        options: Arc<CompilerOptions>,
        context: Utf8PathBuf,
        plugins: Arc<PluginDriver>,
    ) -> Self {
        let (todo_tx, todo_rx) = unbounded_channel();
        let resolver_factory = Arc::new(ResolverFactory::new_with_base_option(
            options.resolve.clone(),
        ));
        let module_factory = Arc::new(NormalModuleFactory {
            options: options.clone(),
            resolver_factory: resolver_factory.clone(),
            context: context.clone(),
        });
        Self {
            options: options.clone(),
            context,
            resolver_factory: resolver_factory.clone(),
            module_factory,
            plugin_driver: plugins,
            working_tasks: Default::default(),
            todo_rx,
            todo_tx,
            dependency_cache: Default::default(),
        }
    }
    // add entries
    pub async fn from_entries(&mut self, memory_manager: &MemoryManager) -> ScannerResult {
        let mut scanner_result = ScannerResult::new();
        let entry_ids = self
            .options
            .entry
            .iter()
            .map(|entry| {
                let entry_dep: BoxDependency = Box::new(EntryDependency::new(
                    entry.import.clone(),
                    self.options.context.clone(),
                ));
                scanner_result.entries.insert(
                    entry.name.clone(),
                    EntryData {
                        name: Some(entry.name.clone()),
                        dependencies: vec![entry_dep.id()],
                    },
                );
                entry_dep
            })
            .collect::<Vec<_>>();
        let start = Instant::now();
        let entry_ids: Vec<_> = entry_ids
            .iter()
            .map(|dep| {
                memory_manager.alloc_dependency(dep.clone())
            })
            .collect();
        eprintln!("start first scan");

        self.build_loop(&mut scanner_result, entry_ids.clone(), memory_manager)
            .await;
        let duration = start.elapsed();

        eprintln!("first scan in {:?} with {} modules", duration,scanner_result._modules.len());

        sleep(Duration::from_secs(2));
        let start = Instant::now();
        
        eprintln!("start second scan");
        let mut new_scanner_result = ScannerResult::new();
        self.build_loop(&mut new_scanner_result, entry_ids.clone(), memory_manager)
            .await;
        let duration = start.elapsed();
        eprintln!("second scan in {:?} with {} modules", duration,scanner_result._modules.len());
        let graph = new_scanner_result.module_graph;
        let start = Instant::now();
        let graph2 = graph.clone();
        let duration = start.elapsed();
        eprintln!("clone graph in {:?}", duration);

        scanner_result
    }
    pub fn handle_module_creation(
        &self,
        dependencies: Vec<DependencyId>,
        origin_module_id: Option<ModuleId>,
        context: Option<Utf8PathBuf>,
        todo_tx: Sender<Result<Task>>,
    ) -> Vec<Task> {
        return vec![(Task::Factorize(FactorizeTask {
                dependencies,
                origin_module_id,
                origin_module_context: context.clone(),
            }))];
    }
    pub fn resolve_module() {}
}

#[derive(Debug)]
pub struct ScannerResult {
    pub _modules: FxHashMap<Ustr, ModuleId>,
    pub collect_modules: Vec<ModuleId>,
    pub module_graph: ModuleGraph,
    pub diagnostics: Diagnostics,
    pub entries: IndexMap<String, EntryData>,
    // means job which doesn't have result yet
    pub remaining: AtomicU32,
}
impl ScannerResult {
    fn add_diagnostic(&self, diag: Report) {
        self.diagnostics.write().unwrap().push(diag);
    }
}
impl ScannerResult {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ScannerResult {
    fn default() -> Self {
        Self {
            _modules: Default::default(),
            collect_modules: Default::default(),
            module_graph: Default::default(),
            diagnostics: Default::default(),
            entries: Default::default(),
            remaining: 0.into(),
        }
    }
}
/// main loop task
impl ModuleScanner {
    // handle_module_creation -> factorize -> add_module -> build -> process_deps -> handle_module_creation
    pub async fn build_loop(
        &mut self,
        state: &mut ScannerResult,
        dependencies: Vec<DependencyId>,
        memory_manager: &MemoryManager,
    ) {
        let mut tasks = vec![];
        dependencies.into_iter().for_each(|dep| {
            // kick off entry dependencies to task_queue
            let task = self.handle_module_creation(
                vec![dep],
                None,
                Some(self.context.clone()),
                self.todo_tx.clone(),
            );
            tasks.extend(task);

        });
        while let Some(task) = tasks.pop() {
            let more_tasks = self.handle_task(task, state,memory_manager).await;
            tasks.extend(more_tasks)
        }
    }

    async fn handle_task(
        &mut self,
        task: Task,
        state: &mut ScannerResult,
        memory_manager: &MemoryManager,
    ) -> Vec<Task> {
        match task {
            Task::Factorize(factorize_task) => {
                let original_module = factorize_task
                    .origin_module_id
                    .map(|x| memory_manager.module_by_id(x));
                let original_module_context =
                    original_module.and_then(|x| x.read().get_context().map(|x| x.to_path_buf()));
                let tx = self.todo_tx.clone();
                ({
                    let compiler_id = COMPILER_CONTEXT.get();
                    let options = self.options.clone();
                    let plugin_driver = self.plugin_driver.clone();
                    let module_factory = self.module_factory.clone();
                    let dependency_cache = self.dependency_cache.clone();
                    // unsafe convert memory_manager to 'static
                    let memory_manager = unsafe { &*(memory_manager as *const MemoryManager) };
                    return  ModuleScanner::handle_factorize(
                            tx,
                            factorize_task,
                            original_module_context,
                            options,
                            plugin_driver,
                            module_factory,
                            dependency_cache,
                            memory_manager
                        ).await;
                });
            }
            Task::Build(task) => {
                if state
                    ._modules
                    .contains_key(&task.module.read().identifier())
                {
                    return vec![]
                };

                let sender = self.todo_tx.clone();
                // unsafe convert memory_manager to 'staticg
                let memory_manager = unsafe { &*(memory_manager as *const MemoryManager) };
                ({
                    let options = self.options.clone();
                    let plugin_driver = self.plugin_driver.clone();
                    let compiler_id = COMPILER_CONTEXT.get();
                    return ModuleScanner::handle_build(task, options, plugin_driver, sender,memory_manager).await;
                });
            }
            Task::AddModule(task) => {
                return self.handle_add_module_and_dependencies(state, task, memory_manager);
            }
        }
    }
    #[instrument("handle_factorize", skip_all)]
    async fn handle_factorize(
        tx: Sender<Result<Task>>,
        task: FactorizeTask,
        original_module_context: Option<Utf8PathBuf>,
        options: Arc<CompilerOptions>,
        plugin_driver: Arc<PluginDriver>,
        module_factory: Arc<NormalModuleFactory>,
        dependency_cache: Arc<DashMap<DependencyId, WritableModule>>,
        memory_manager: &MemoryManager
    ) -> Vec<Task> {
        let module_dependency_id = task.dependencies[0].clone();
        let module_dependency = memory_manager.dependency_by_id(module_dependency_id);
        let context = if let Some(context) = module_dependency.get_context() {
            context.to_owned()
        } else if let Some(context) = original_module_context {
            context.to_owned()
        } else {
            options.context.clone()
        };
        let module_dependency = module_dependency.clone();
        if let Some(reference) = dependency_cache.get(&module_dependency.id()) {
            let module = reference.clone();
            return vec![(Task::Build(BuildTask {
                origin_module_id: task.origin_module_id,
                module,
                dependencies: task.dependencies.clone(),
            }))]
        }
        if let Some(reference) = dependency_cache.get(&module_dependency.id()) {
            let module = reference.clone();
            return vec![(Task::Build(BuildTask {
                origin_module_id: task.origin_module_id,
                module,
                dependencies: task.dependencies.clone(),
            }))]
        }
        match module_factory
            .create(
                ModuleFactoryCreateData {
                    module_dependency: module_dependency.clone(),
                    context,
                    options: options.clone(),
                },
                plugin_driver.clone(),
            )
            .await
        {
            Ok(factory_result) => {
                let module: WritableModule = RwCell::new(Box::new(factory_result.module));
                
                dependency_cache.insert(module_dependency.id(), module.clone());
                return vec!(Task::Build(BuildTask {
                    origin_module_id: task.origin_module_id,
                    module,
                    dependencies: task.dependencies.clone(),
                }))
            }
            Err(err) => {
               return vec![]
            }
        }
    }
    #[instrument("handle_add_module_and_dependencies", skip_all)]
    fn handle_add_module_and_dependencies(
        &self,
        state: &mut ScannerResult,
        task: AddModuleTask,
        memory_manager: &MemoryManager,
    ) -> Vec<Task> {
        let module = task.module;

        let original_module_context = module.read().get_context().map(|x| x.to_owned());
        let identifier = module.read().identifier();
        let module_id = memory_manager.alloc_module(module);

        state.collect_modules.push(module_id);
        state._modules.insert(identifier, module_id);
        // update origin -> self
        state.module_graph.set_resolved_module(
            task.origin_module_id,
            task.module_dependency,
            module_id,
            memory_manager,
        );
        let mut sorted_dependencies: HashMap<String, DependencyId, _> = HashMap::new();
        for dep_id in task.dependencies {
            let dep = memory_manager.dependency_by_id(dep_id);
            if let Some(module_dependency) = dep.as_module_dependency() {
                sorted_dependencies.insert(module_dependency.resource_identifier(), dep_id);
            }
        }
        let mut tasks = vec![];
        for dep in sorted_dependencies.into_values() {
            tasks.extend(self.handle_module_creation(
                vec![dep],
                Some(module_id),
                original_module_context.clone(),
                self.todo_tx.clone(),
            ));
        }
        return tasks;
    }
    #[instrument("handle_build", skip_all)]
    async fn handle_build(
        task: BuildTask,
        options: Arc<CompilerOptions>,
        plugin_driver: Arc<PluginDriver>,
        todo_tx: Sender<Result<Task>>,
        memory_manager: &MemoryManager
    ) -> Vec<Task> {
        let module = task.module.clone();
        let module_dependency = task.dependencies[0].clone();

        if (module.read().need_build()) {
            let mut write_module = module.write();
            write_module
                .build(BuildContext {
                    options: options.clone(),
                    plugin_driver: plugin_driver.clone(),
                }, memory_manager)
                .await;
        };
        let module = task.module.clone();
        let dependencies: Vec<DependencyId> = module.read().get_dependencies();
        return vec![(Task::AddModule(AddModuleTask {
                dependencies,
                origin_module_id: task.origin_module_id,
                module_dependency,
                module,
            }))]
    }
}
