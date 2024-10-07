use std::slice::Iter;

use index_vec::IndexVec;
use indexmap::IndexMap;
use rustc_hash::FxHashMap;
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
    pub dependency_to_connection: IndexMap<DependencyId, ConnectionId>,
    pub module_id_to_module_graph_module_id: FxHashMap<ModuleId, ModuleGraphModuleId>
}

impl ModuleGraph {
    pub fn module_id_by_dependency_id(&self, dep_id: DependencyId) -> ModuleId {
        let connection_id = self.dependency_to_connection.get(&dep_id).expect("get connection failed");
        let connection = self.connection_by_id(*connection_id);
        connection.resolved_module_id
    }
    pub fn set_resolved_module(&mut self, origin_module_id: Option<ModuleId>, dep_id: DependencyId, resolved_module_id: ModuleId) {
        let connection = Connection::new(
            origin_module_id,
            resolved_module_id
        );
        let connection_id = self.add_connection(connection);
        self.dependency_to_connection.insert(dep_id, connection_id);
        let resolved_mgm_id = self.module_graph_module_id_by_module_id(resolved_module_id);
        let resolved_module = self.module_graph_module_by_id_mut(resolved_mgm_id);
        resolved_module.add_incoming_connection(connection_id);
        
        if let Some(origin_module_id) = origin_module_id {
            let mgm_id = self.module_graph_module_id_by_module_id(origin_module_id);
            let mgm = self.module_graph_module_by_id_mut(mgm_id);
            mgm.add_outgoing_connection(connection_id);
        }

    }
    pub fn module_graph_module_id_by_module_id(&mut self, module_id: ModuleId) -> ModuleGraphModuleId {
        let mgm_id = if let Some(&id) = self.module_id_to_module_graph_module_id.get(&module_id) {
            id
        } else {
            let mgm = ModuleGraphModule::new();
            let new_id = self.add_module_graph_module(mgm);
            self.module_id_to_module_graph_module_id.insert(module_id, new_id);
            new_id
        };
        mgm_id
    }
    pub fn get_outgoing_connections(&mut self, module_id: ModuleId)-> Vec<ConnectionId>{
        let mgm_id = self.module_graph_module_id_by_module_id(module_id);
        let mgm = self.module_graph_module_by_id(mgm_id);
        mgm.outgoing_connections.clone()

    }
}
