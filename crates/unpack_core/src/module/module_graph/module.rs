use crate::module::{BoxModule, ModuleId};

use super::ModuleGraph;

impl ModuleGraph {
    pub fn add_module(&mut self, module: BoxModule) -> ModuleId {
        self.modules.push(module)
    }
    pub fn module_by_id(&self, id: ModuleId) -> &BoxModule {
        &self.modules[id]
    }
    pub fn module_by_id_mut(&mut self, id: ModuleId) -> &mut BoxModule {
        &mut self.modules[id]
    }
}
