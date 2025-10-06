use indexmap::IndexMap;

use crate::{
    dependency::{BoxDependency, DependencyId},
    memory_manager::arena::{Arena, Idx},
    module::{BoxModule, Connection, ConnectionId, ModuleGraphModule},
};

pub mod arena;

#[derive(Default, Debug)]
pub struct MemoryManager {
    modules: Arena<BoxModule>,
    dependencies: IndexMap<DependencyId, BoxDependency>,
    connections: Arena<Connection>,
    module_graph_modules: Arena<ModuleGraphModule>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            modules: Default::default(),
            dependencies: Default::default(),
            connections: Default::default(),
            module_graph_modules: Default::default(),
        }
    }
    pub fn alloc_module(&mut self, module: BoxModule) -> Idx<BoxModule> {
        self.modules.insert(module)
    }
    pub fn module_by_id(&self, id: Idx<BoxModule>) -> &BoxModule {
        &self.modules[id]
    }
    pub fn alloc_dependency(&mut self, dep: BoxDependency) -> DependencyId {
        let dep_id = dep.id();
        self.dependencies.insert(dep_id, dep);
        dep_id
    }
    // get dependency by id
    pub fn dependency_by_id(&self, id: DependencyId) -> &BoxDependency {
        self.dependencies.get(&id).expect("get dependency failed")
    }
    pub fn dependency_by_id_mut(&mut self, id: DependencyId) -> &mut BoxDependency {
        self.dependencies
            .get_mut(&id)
            .expect("get depependency failed")
    }
     pub fn add_connection(&mut self, connection: Connection) -> ConnectionId {
        self.connections.insert(connection)
    }
    pub fn connection_by_id(&self, connection_id: ConnectionId) -> &Connection {
        &self.connections[connection_id]
    }
}
