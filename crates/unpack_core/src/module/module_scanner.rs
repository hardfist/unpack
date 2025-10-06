use crate::dependency::{BoxDependency, DependencyId, ModuleDependency};
use crate::errors::miette::{Report, Result};
use crate::errors::Diagnostics;
use crate::memory_manager::arena::Idx;
use crate::memory_manager::MemoryManager;
use crate::module::{BoxModule, BuildContext, ModuleId};
use crate::normal_module_factory::{ModuleFactoryCreateData, NormalModuleFactory};
use crate::plugin::PluginDriver;
use crate::scheduler::COMPILER_CONTEXT;
use crate::task::{AddModuleTask, BuildTask, FactorizeTask};
use crate::{resolver_factory::ResolverFactory, task::Task};
use camino::Utf8PathBuf;
use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::instrument;

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
        }
    }
    // add entries
    pub async fn from_entries(&mut self, memory_manager: &mut MemoryManager) -> ScannerResult {
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

        self.build_loop(&mut scanner_result, entry_ids, memory_manager)
            .await;
        scanner_result
    }
    pub fn handle_module_creation(
        &self,
        dependencies: Vec<BoxDependency>,
        origin_module_id: Option<ModuleId>,
        context: Option<Utf8PathBuf>,
        todo_tx: Sender<Result<Task>>,
    ) {
        dependencies.into_iter().for_each(|dep| {
            todo_tx
                .send(Ok(Task::Factorize(FactorizeTask {
                    dependencies: vec![dep],
                    origin_module_id,
                    origin_module_context: context.clone(),
                })))
                .unwrap();
        });
    }
    pub fn resolve_module() {}
}

#[derive(Debug)]
pub struct ScannerResult {
    pub _modules: FxHashMap<String, Idx<BoxModule>>,
    pub module_graph: ModuleGraph,
    pub diagnostics: Diagnostics,
    pub entries: IndexMap<String, EntryData>,
    // means job which doesn't have result yet
    pub remaining: AtomicU32,
}
impl ScannerResult {
    fn add_diagnostic(&mut self, diag: Report) {
        self.diagnostics.push(diag);
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
        dependencies: Vec<BoxDependency>,
        memory_manager: &mut MemoryManager,
    ) {
        // kick off entry dependencies to task_queue
        self.handle_module_creation(
            dependencies,
            None,
            Some(self.context.clone()),
            self.todo_tx.clone(),
        );
        loop {
            tokio::select! {
                task = self.todo_rx.recv() => {
                    if let Some(task) = task {
                        match task {
                            Ok(task) =>{
                               self.handle_task(task, state,memory_manager);
                            },
                            Err(err) => {
                                state.add_diagnostic(err);
                            }
                        }

                    }
                }
                task = self.working_tasks.join_next() => {
                    if let Some(handle) = task {
                        if handle.is_err() {
                            panic!("unexpected spawn error");
                        }

                    } else if self.todo_rx.is_empty() {
                        // if todo_task and working_task both empty which mean we can safely exit

                        break;
                    }
                }

            }
        }
    }

    fn handle_task(
        &mut self,
        task: Task,
        state: &mut ScannerResult,
        memory_manager: &mut MemoryManager,
    ) {
        match task {
            Task::Factorize(factorize_task) => {
                let original_module = factorize_task
                    .origin_module_id
                    .map(|x| memory_manager.module_by_id(x));
                let original_module_context =
                    original_module.and_then(|x| x.get_context().map(|x| x.to_path_buf()));
                let tx = self.todo_tx.clone();
                self.working_tasks.spawn({
                    let compiler_id = COMPILER_CONTEXT.get();
                    let options = self.options.clone();
                    let plugin_driver = self.plugin_driver.clone();
                    let module_factory = self.module_factory.clone();
                    COMPILER_CONTEXT.scope(compiler_id, async move {
                        ModuleScanner::handle_factorize(
                            tx,
                            factorize_task,
                            original_module_context,
                            options,
                            plugin_driver,
                            module_factory,
                        )
                        .await;
                    })
                });
            }
            Task::Build(task) => {
                if state._modules.contains_key(task.module.identifier()) {
                    return;
                };

                let sender = self.todo_tx.clone();

                self.working_tasks.spawn({
                    let options = self.options.clone();
                    let plugin_driver = self.plugin_driver.clone();
                    let compiler_id = COMPILER_CONTEXT.get();
                    COMPILER_CONTEXT.scope(compiler_id, async move {
                        ModuleScanner::handle_build(task, options, plugin_driver, sender).await;
                    })
                });
            }
            Task::AddModule(task) => {
                self.handle_add_module_and_dependencies(state, task, memory_manager);
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
    ) {
        let module_dependency = task.dependencies[0].clone();

        let context = if let Some(context) = module_dependency.get_context() {
            context.to_owned()
        } else if let Some(context) = original_module_context {
            context.to_owned()
        } else {
            options.context.clone()
        };
        let module_dependency = module_dependency.clone();
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
                let module = Box::new(factory_result.module);
                tx.send(Ok(Task::Build(BuildTask {
                    origin_module_id: task.origin_module_id,
                    module,
                    dependencies: task.dependencies.clone(),
                })))
                .unwrap();
            }
            Err(err) => {
                tx.send(Err(err)).unwrap();
            }
        }
    }
    #[instrument("handle_add_module_and_dependencies", skip_all)]
    fn handle_add_module_and_dependencies(
        &self,
        state: &mut ScannerResult,
        task: AddModuleTask,
        memory_manager: &mut MemoryManager,
    ) {
        let module = task.module;
        let original_module_context = module.get_context().map(|x| x.to_owned());
        let identifier = module.identifier().to_string();
        let module_id = memory_manager.alloc_module(module);
        state.module_graph.add_module(module_id);
        let dependency_id = memory_manager.alloc_dependency(task.module_dependency);
        state._modules.insert(identifier.to_string(), module_id);
        // update origin -> self
        state
            .module_graph
            .set_resolved_module(task.origin_module_id, dependency_id, module_id);
        let mut sorted_dependencies: HashMap<String, BoxDependency, _> = HashMap::new();
        for dep in task.dependencies {
            if let Some(module_dependency) = dep.as_module_dependency() {
                sorted_dependencies.insert(module_dependency.resource_identifier(), dep);
            }
        }
        for dep in sorted_dependencies.into_values() {
            self.handle_module_creation(
                vec![dep],
                Some(module_id),
                original_module_context.clone(),
                self.todo_tx.clone(),
            );
        }
    }
    #[instrument("handle_build", skip_all)]
    async fn handle_build(
        task: BuildTask,
        options: Arc<CompilerOptions>,
        plugin_driver: Arc<PluginDriver>,
        todo_tx: Sender<Result<Task>>,
    ) {
        let mut module = task.module;
        let module_dependency = task.dependencies[0].clone();

        match module
            .build(BuildContext {
                options: options.clone(),
                plugin_driver: plugin_driver.clone(),
            })
            .await
        {
            Ok(result) => {
                todo_tx
                    .send(Ok(Task::AddModule(AddModuleTask {
                        dependencies: result.module_dependencies,
                        origin_module_id: task.origin_module_id,
                        module_dependency,
                        module,
                    })))
                    .unwrap();
            }
            Err(err) => {
                todo_tx.send(Err(err)).unwrap();
            }
        };
    }
}
