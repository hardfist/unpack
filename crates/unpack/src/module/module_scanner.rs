use crate::dependency::BoxDependency;
use crate::errors::miette::{Report, Result};
use crate::errors::Diagnostics;
use crate::module::{BuildContext, ModuleId};
use crate::normal_module_factory::{ModuleFactoryCreateData, NormalModuleFactory};
use crate::plugin::PluginDriver;
use crate::task::{BuildTask, FactorizeTask, ProcessDepsTask};
use crate::{resolver_factory::ResolverFactory, task::Task};
use camino::Utf8PathBuf;
use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use tokio::runtime::Handle;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use super::module_graph::ModuleGraph;
use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use crate::{compiler::CompilerOptions, dependency::EntryDependency};
#[derive(Debug)]
pub struct EntryData {
    name: Option<String>,
}
#[derive(Debug, Clone)]
pub struct ModuleScanner {
    options: Arc<CompilerOptions>,
    context: Utf8PathBuf,
    resolver_factory: Arc<ResolverFactory>,
    module_factory: Arc<NormalModuleFactory>,
    plugin_driver: Arc<PluginDriver>
}
struct FactorizeParams {}
impl ModuleScanner {
    pub fn new(
        options: Arc<CompilerOptions>,
        context: Utf8PathBuf,
        plugins: Arc<PluginDriver>
    ) -> Self {
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
            plugin_driver: plugins
        }
    }
    // add entries
    pub async fn add_entries(&self, state: &mut ScannerState, recv: &mut Receiver<Result<Task>>) {
        let entry_ids = self
            .options
            .entry
            .iter()
            .map(|entry| {
                let entry_dep: BoxDependency = Box::new(EntryDependency::new(
                    entry.import.clone(),
                    self.options.context.clone(),
                ));
                state.entries.insert(
                    entry.name.clone(),
                    EntryData {
                        name: Some(entry.name.clone()),
                    },
                );
                entry_dep
            })
            .collect::<Vec<_>>();

        self.build_loop(state, entry_ids,recv).await
    }
    pub fn handle_module_creation(
        &self,
        state: &mut ScannerState,
        dependencies: Vec<BoxDependency>,
        origin_module_id: Option<ModuleId>,
        context: Option<Utf8PathBuf>,
    ) {
        dependencies.into_iter().for_each(|dep| {
            state.add_remaining_result();
            state
                .tx
                .send(Ok(Task::Factorize(FactorizeTask {
                    module_dependency: dep,
                    origin_module_id,
                    origin_module_context: context.clone(),
                })))
                .unwrap();
        });
    }
    pub fn resolve_module() {}
}

#[derive(Debug)]
pub struct ScannerState {
    _modules: FxHashMap<String, ModuleId>,
    pub module_graph: ModuleGraph,
    pub tx: Sender<Result<Task>>,
    pub diagnostics:Diagnostics,
    pub entries: IndexMap<String, EntryData>,
    // means job which doesn't have result yet
    pub remaining: AtomicU32,
}
impl ScannerState {
    fn add_remaining_result(&self) {
        self.remaining
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    fn sub_remaining_result(&self) {
        self.remaining
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
    fn get_remaining_result(&self) -> u32 {
        self.remaining.load(std::sync::atomic::Ordering::SeqCst)
    }
    fn add_diagnostic(&mut self, diag: Report) {
        self.diagnostics.push(diag);
    }
}
impl ScannerState {
    pub fn new(tx: Sender<Result<Task>>) -> Self {
        Self {
            tx,
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
    pub async fn build_loop(&self, state: &mut ScannerState, dependencies: Vec<BoxDependency>, recv: &mut Receiver<Result<Task>>) {
        // kick off entry dependencies to task_queue
        self.handle_module_creation(state, dependencies, None, Some(self.context.clone()));
        while state.get_remaining_result() > 0 {
            let task = recv.recv().await.unwrap();
            state.sub_remaining_result();
            match task {
                Ok(task) => {
                    self.handle_task(task, state);
                }
                Err(err) => {
                    state.add_diagnostic(err);
                }
            }
        }
    }

    fn handle_task(&self, task: Task, state: &mut ScannerState) {
        match task {
            Task::Factorize(factorize_task) => {
                let original_module = factorize_task
                    .origin_module_id
                    .map(|x| state.module_graph.module_by_id(x));
                let original_module_context =
                    original_module.and_then(|x| x.get_context().map(|x| x.to_path_buf()));
                state.add_remaining_result();
                let tx = state.tx.clone();
                let scanner = self.clone();

                Handle::current().spawn(async {
                    Self::handle_factorize(scanner, tx, factorize_task, original_module_context).await;
                });
            }
            Task::Build(task) => {
                let scanner = self.clone();
                if state._modules.contains_key(task.module.identifier()) {
                    return;
                };
                state.add_remaining_result();
                let sender = state.tx.clone();
                Handle::current().spawn(async move  {
                    Self::handle_build(scanner, sender, task).await;
                });
            }
            Task::ProcessDeps(task) => {
                self.handle_process_deps(state, task);
            }
        }
    }
    async fn handle_factorize(
        self,
        tx: Sender<Result<Task>>,
        task: FactorizeTask,
        original_module_context: Option<Utf8PathBuf>,
    ) {
        let module_dependency = task.module_dependency.clone();

        let context = if let Some(context) = module_dependency.get_context() {
            context.to_owned()
        } else if let Some(context) = original_module_context {
            context.to_owned()
        } else {
            self.options.context.clone()
        };
        let module_dependency = module_dependency.clone();
        match self.module_factory.create(ModuleFactoryCreateData {
            module_dependency: module_dependency.clone(),
            context,
            options: self.options.clone(),
            
        }, self.plugin_driver.clone()).await {
            Ok(factory_result) => {
                let module = Box::new(factory_result.module);
                tx.send(Ok(Task::Build(BuildTask {
                    origin_module_id: task.origin_module_id,
                    module,
                    module_dependency: task.module_dependency.clone(),
                })))
                .unwrap();
            }
            Err(err) => {
                tx.send(Err(err)).unwrap();
            }
        }
    }
    fn handle_process_deps(&self, state: &mut ScannerState, task: ProcessDepsTask) {
        let module = task.module;
        let original_module_context = module.get_context().map(|x| x.to_owned());
        let identifier = module.identifier().to_string();
        let module_id = state.module_graph.add_module(module);
        let dependency_id = state.module_graph.add_dependency(task.module_dependency);
        state._modules.insert(identifier.to_string(), module_id);
        // update origin -> self
        state
            .module_graph
            .set_resolved_module(task.origin_module_id, dependency_id, module_id);
        self.handle_module_creation(
            state,
            task.dependencies,
            Some(module_id),
            original_module_context,
        );
    }
    async fn handle_build(self, tx: Sender<Result<Task>>, task: BuildTask) {
        let mut module = task.module;
        let module_dependency = task.module_dependency;

        match module.build(BuildContext {
            options: self.options.clone(),
            plugin_driver: self.plugin_driver.clone(),
        }).await {
            Ok(result) => {
                tx.send(Ok(Task::ProcessDeps(ProcessDepsTask {
                    dependencies: result.module_dependencies,
                    origin_module_id: task.origin_module_id,
                    module_dependency,
                    module,
                })))
                .unwrap();
            }
            Err(err) => {
                tx.send(Err(err)).unwrap();
            }
        };
    }
}
