use index_vec::define_index_type;

use super::ConnectionId;

#[derive(Debug)]
pub struct ModuleGraphModule {
    incoming_connections: Vec<ConnectionId>,
    outgoing_connections: Vec<ConnectionId>
}
impl ModuleGraphModule {
    pub fn new() -> Self{
        Self {
            incoming_connections: Default::default(),
            outgoing_connections: Default::default()
        }
    }
    pub fn add_incoming_connection(&mut self, connection_id: ConnectionId) {
        self.incoming_connections.push(connection_id);
    }
    pub fn add_outgoing_connection(&mut self, connection_id: ConnectionId) {
        self.outgoing_connections.push(connection_id)
    }
}
define_index_type! {
    pub struct ModuleGraphModuleId = u32;
}