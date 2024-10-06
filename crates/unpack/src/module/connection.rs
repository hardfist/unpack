use super::ModuleId;
use index_vec::define_index_type;
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

define_index_type! {
    pub struct ConnectionId = u32;
}
