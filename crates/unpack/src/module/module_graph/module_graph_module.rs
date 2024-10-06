use crate::module::{ModuleGraphModule, ModuleGraphModuleId};

use super::ModuleGraph;

impl ModuleGraph {
    pub fn add_module_graph_module(&mut self, mgm: ModuleGraphModule) -> ModuleGraphModuleId{
        self.module_graph_modules.push(mgm)
    }
}