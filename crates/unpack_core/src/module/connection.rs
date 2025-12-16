use crate::memory_manager::arena::Idx;

use super::ModuleId;
#[derive(Debug, Clone)]
pub struct Connection {
    pub origin_module_id: Option<ModuleId>,
    pub resolved_module_id: ModuleId,
}
impl Connection {
    pub fn new(origin_module_id: Option<ModuleId>, resolved_module_id: ModuleId) -> Self {
        Self {
            origin_module_id,
            resolved_module_id,
        }
    }
}
pub type ConnectionId = Idx<Connection>;
