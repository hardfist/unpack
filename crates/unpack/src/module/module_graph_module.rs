use index_vec::define_index_type;

use super::ConnectionId;

#[derive(Debug)]
pub struct ModuleGraphModule {
    incoming_connections: Vec<ConnectionId>,
    outgoing_connections: Vec<ConnectionId>
}
define_index_type! {
    pub struct ModuleGraphModuleId = u32;
}