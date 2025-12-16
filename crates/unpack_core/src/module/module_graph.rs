use crate::{dependency::DependencyId, memory_manager::MemoryManager, module::ModuleId};

use super::{Connection, ConnectionId, ModuleGraphModule, ModuleGraphModuleId};

#[derive(Debug, Default,Clone)]
pub struct ModuleGraph {
    pub dependency_to_connection: im::OrdMap<DependencyId, ConnectionId>,
    pub module_id_to_module_graph_module_id: im::HashMap<ModuleId, ModuleGraphModuleId>,
}

impl ModuleGraph {
    #[allow(unused_variables)]
    pub fn new_from_entries(entries: Vec<DependencyId>, memory_manager: MemoryManager) {
        let module_graph = ModuleGraph::default();
        let mut queue: Vec<(DependencyId, Option<ModuleId>)> =
            entries.into_iter().map(|id| (id, None)).collect();
        //let origin_module_id = None;
        while let Some((dep, module_id)) = queue.pop() {
            let origin_module = module_id.map(|id| memory_manager.module_by_id(id));
        }
    }
}

impl ModuleGraph {
    pub fn module_id_by_dependency_id(
        &self,
        dep_id: DependencyId,
        memory_manager: &MemoryManager,
    ) -> ModuleId {
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
        memory_manager: &MemoryManager,
    ) {
        let connection = Connection::new(origin_module_id, resolved_module_id);
        let connection_id = memory_manager.alloc_connection(connection);
        self.dependency_to_connection.insert(dep_id, connection_id);
        self.module_graph_module_id_by_module_id(resolved_module_id, memory_manager, |mut mgm| {
            mgm.add_incoming_connection(connection_id);
            mgm
        });

        if let Some(origin_module_id) = origin_module_id {
            self.module_graph_module_id_by_module_id(
                origin_module_id,
                memory_manager,
                |mut mgm| {
                    mgm.add_outgoing_connection(connection_id);
                    mgm
                },
            );
        }
    }
    pub fn module_graph_module_id_by_module_id(
        &mut self,
        module_id: ModuleId,
        memory_manager: &MemoryManager,
        update_module_graph_module: impl Fn(ModuleGraphModule) -> ModuleGraphModule,
    ) -> ModuleGraphModuleId {
        let mgm_id = if let Some(&id) = self.module_id_to_module_graph_module_id.get(&module_id) {
            id
        } else {
            let mgm = update_module_graph_module(ModuleGraphModule::new());
            let new_id = memory_manager.alloc_module_graph_module(mgm);
            self.module_id_to_module_graph_module_id
                .insert(module_id, new_id);
            new_id
        };
        mgm_id
    }
    pub fn get_outgoing_connections(
        &mut self,
        module_id: ModuleId,
        memory_manager: &MemoryManager,
    ) -> Vec<ConnectionId> {
        let mgm_id = self.module_graph_module_id_by_module_id(module_id, memory_manager, |mgm| mgm);
        let mgm = memory_manager.module_graph_module_by_id(mgm_id);
        mgm.outgoing_connections.clone()
    }
}
