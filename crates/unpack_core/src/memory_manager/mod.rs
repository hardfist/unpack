use indexmap::IndexMap;

use crate::{
    dependency::{BoxDependency, DependencyId},
    memory_manager::arena::{Arena, Idx},
    module::{BoxModule, Connection, ConnectionId, ModuleGraphModule, ModuleGraphModuleId},
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
}
// don't expose mutable borrow of arena's item
impl MemoryManager {
   
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
    pub fn dependency_by_id(&self, id: DependencyId) -> &BoxDependency {
        self.dependencies.get(&id).expect("get dependency failed")
    }
     pub fn alloc_connection(&mut self, connection: Connection) -> ConnectionId {
        self.connections.insert(connection)
    }
    pub fn connection_by_id(&self, connection_id: ConnectionId) -> &Connection {
        &self.connections[connection_id]
    }
    pub fn alloc_module_graph_module(&mut self, mgm: ModuleGraphModule) -> ModuleGraphModuleId {
        self.module_graph_modules.insert(mgm)
    }
    pub fn module_graph_module_by_id(&self, mgm_id: ModuleGraphModuleId) -> &ModuleGraphModule {
        &self.module_graph_modules[mgm_id]
    }
}
