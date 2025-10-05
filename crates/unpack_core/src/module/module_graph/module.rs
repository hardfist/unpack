use crate::module::ModuleId;

use super::ModuleGraph;
impl ModuleGraph {
    pub fn add_module(&mut self, module_id: ModuleId) {
        self.modules.push(module_id)
    }
}
