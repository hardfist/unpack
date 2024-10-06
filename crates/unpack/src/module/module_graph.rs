use index_vec::IndexVec;
use indexmap::IndexMap;
mod connection;
mod dependency;
mod module;
mod module_graph_module;
use crate::{
    dependency::{BoxDependency, DependencyId},
    module::{BoxModule, ModuleId},
};

use super::{Connection, ConnectionId, ModuleGraphModule, ModuleGraphModuleId};

#[derive(Debug, Default)]
pub struct ModuleGraph {
    pub dependencies: IndexVec<DependencyId, BoxDependency>,
    pub modules: IndexVec<ModuleId, BoxModule>,
    pub module_graph_modules: IndexVec<ModuleGraphModuleId, ModuleGraphModule>,
    pub connections: IndexVec<ConnectionId, Connection>,
    pub dep_to_connection: IndexMap<DependencyId, ConnectionId>,
    pub module_id_to_module_graph_module_id: IndexMap<ModuleId, ModuleGraphModule>
}

impl ModuleGraph {
    pub fn module_id_by_dependency_id(&self, dep_id: DependencyId) -> ModuleId {
        let connection_id = self.dep_to_connection.get(&dep_id).expect("get connection failed");
        let connection = self.connection_by_id(*connection_id);
        connection.resolved_module_id
    }
    pub fn set_resolved_module(&mut self, origin_module_id: Option<ModuleId>, dep_id: DependencyId, resolved_module_id: ModuleId) {
        let connection = Connection::new(
            origin_module_id,
            resolved_module_id
        );
        let connection_id = self.add_connection(connection);
        if let Some(origin_module_id) = origin_module_id {
            
        }

    }
}
