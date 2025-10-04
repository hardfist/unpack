use crate::module::{ModuleGraphModule, ModuleGraphModuleId};

use super::ModuleGraph;

impl ModuleGraph {
    pub fn add_module_graph_module(&mut self, mgm: ModuleGraphModule) -> ModuleGraphModuleId {
        self.module_graph_modules.push(mgm)
    }
    pub fn module_graph_module_by_id_mut(
        &mut self,
        mgm_id: ModuleGraphModuleId,
    ) -> &mut ModuleGraphModule {
        &mut self.module_graph_modules[mgm_id]
    }
    pub fn module_graph_module_by_id(&self, mgm_id: ModuleGraphModuleId) -> &ModuleGraphModule {
        &self.module_graph_modules[mgm_id]
    }
}
