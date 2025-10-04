use crate::module::{Connection, ConnectionId};

use super::ModuleGraph;

impl ModuleGraph {
    pub fn add_connection(&mut self, connection: Connection) -> ConnectionId {
        self.connections.push(connection)
    }
    pub fn connection_by_id(&self, connection_id: ConnectionId) -> &Connection {
        &self.connections[connection_id]
    }
}
