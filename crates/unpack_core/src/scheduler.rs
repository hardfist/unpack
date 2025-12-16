use std::{
    future::Future,
    sync::{atomic::AtomicU32, Arc},
};

use tokio::{task::JoinHandle, task_local};

use crate::memory_manager::MemoryManager;
static COMPILER_ID_GENERATOR: AtomicU32 = AtomicU32::new(0);
pub struct CompilerContext {
    compiler_id: u32,
    dependency_id: AtomicU32,
    memory_manager: MemoryManager,
}
impl CompilerContext {
    pub fn new() -> Self {
        Self {
            compiler_id: COMPILER_ID_GENERATOR.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            dependency_id: AtomicU32::new(0),
            memory_manager: MemoryManager::new(),
        }
    }
    pub fn get_memory_manager(&self) -> &MemoryManager {
        &self.memory_manager
    }
    pub fn get_compiler_id(&self) -> u32 {
        self.compiler_id
    }
    pub fn fetch_new_dependency_id(&self) -> u32 {
        self.dependency_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

impl Default for CompilerContext {
    fn default() -> Self {
        Self::new()
    }
}

task_local! {
    pub static COMPILER_CONTEXT: Arc<CompilerContext>;
}

fn spawn_in_compiler_scope<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let compiler_id = COMPILER_CONTEXT.get();

    tokio::spawn(COMPILER_CONTEXT.scope(compiler_id, future))
}
