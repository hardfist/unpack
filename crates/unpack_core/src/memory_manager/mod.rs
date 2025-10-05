use crate::{memory_manager::arena::{Arena, Idx}, module::BoxModule};

pub mod arena;

#[derive(Default,Debug)]
pub struct MemoryManager {
    pub modules: Arena<BoxModule>
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            modules: Default::default(),
        }
    }
    pub fn alloc_module(&mut self, module: BoxModule) -> Idx<BoxModule> {
        self.modules.insert(module)
    }
    pub fn module_by_id(&self, id: Idx<BoxModule>) -> &BoxModule {
        &self.modules[id]
    }
}
