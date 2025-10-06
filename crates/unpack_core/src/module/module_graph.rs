
use indexmap::IndexMap;
use rustc_hash::FxHashMap;

use crate::{
    dependency::DependencyId, memory_manager::MemoryManager, module::ModuleId
};

use super::{Connection, ConnectionId, ModuleGraphModule, ModuleGraphModuleId};

#[derive(Debug, Default)]
pub struct ModuleGraph {
    pub dependency_to_connection: IndexMap<DependencyId, ConnectionId>,
    pub module_id_to_module_graph_module_id: FxHashMap<ModuleId, ModuleGraphModuleId>,
}

impl ModuleGraph {
    pub fn module_id_by_dependency_id(&self, dep_id: DependencyId, memory_manager: &MemoryManager) -> ModuleId {
        let connection_id = self
            .dependency_to_connection
            .get(&dep_id)
            .expect("get connection failed");
        let connection = memory_manager.connection_by_id(*connection_id);
        connection.resolved_module_id
    }
    pub fn set_resolved_module(
        &mut self,
        origin_module_id: Option<ModuleId>,
        dep_id: DependencyId,
        resolved_module_id: ModuleId,
        memory_manager: &mut MemoryManager
    ) {
        let connection = Connection::new(origin_module_id, resolved_module_id);
        let connection_id = memory_manager.add_connection(connection);
        self.dependency_to_connection.insert(dep_id, connection_id);
        let resolved_mgm_id = self.module_graph_module_id_by_module_id(resolved_module_id,memory_manager);
        let resolved_module = memory_manager.module_graph_module_by_id_mut(resolved_mgm_id);
        resolved_module.add_incoming_connection(connection_id);

        if let Some(origin_module_id) = origin_module_id {
            let mgm_id = self.module_graph_module_id_by_module_id(origin_module_id,memory_manager);
            let mgm = memory_manager.module_graph_module_by_id_mut(mgm_id);
            mgm.add_outgoing_connection(connection_id);
        }
    }
    pub fn module_graph_module_id_by_module_id(
        &mut self,
        module_id: ModuleId,
        memory_manager: &mut MemoryManager
    ) -> ModuleGraphModuleId {
        let mgm_id = if let Some(&id) = self.module_id_to_module_graph_module_id.get(&module_id) {
            id
        } else {
            let mgm = ModuleGraphModule::new();
            let new_id = memory_manager.add_module_graph_module(mgm);
            self.module_id_to_module_graph_module_id
                .insert(module_id, new_id);
            new_id
        };
        mgm_id
    }
    pub fn get_outgoing_connections(&mut self, module_id: ModuleId, memory_manager: &mut MemoryManager) -> Vec<ConnectionId> {
        let mgm_id = self.module_graph_module_id_by_module_id(module_id,memory_manager);
        let mgm = memory_manager.module_graph_module_by_id(mgm_id);
        mgm.outgoing_connections.clone()
    }
}
