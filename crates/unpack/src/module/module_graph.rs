use index_vec::IndexVec;
use indexmap::IndexMap;
mod module_graph_connection;
mod module_graph_dependency;
mod module_graph_module;
use crate::{
    dependency::{BoxDependency, DependencyId},
    module::{BoxModule, ModuleId},
};

use super::{Connection, ConnectionId};

#[derive(Debug, Default)]
pub struct ModuleGraph {
    pub dependencies: IndexVec<DependencyId, BoxDependency>,
    pub modules: IndexVec<ModuleId, BoxModule>,
    pub connections: IndexVec<ConnectionId, Connection>,
    pub dep2connection: IndexMap<DependencyId, ConnectionId>,
}

impl ModuleGraph {
    pub fn module_id_by_dependency_id(&self, dep_id: DependencyId) -> ModuleId {
        let connection_id = self.dep2connection.get(&dep_id).expect("get connection failed");
        let connection = self.connection_by_id(*connection_id);
        connection.resolved_module_id
    }
}
