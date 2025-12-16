use std::sync::RwLock;

use dashmap::DashMap;
use indexmap::IndexMap;

use crate::{
    dependency::{BoxDependency, DependencyId},
    memory_manager::arena::Arena,
    module::{
        Connection, ConnectionId, ModuleGraphModule, ModuleGraphModuleId, ModuleId, ReadonlyModule,
    },
};

pub mod arena;
pub mod intern;

#[derive(Default, Debug)]
pub struct MemoryManager {
    module_caches: DashMap<ustr::Ustr, ReadonlyModule>,
    dependencies: RwLock<IndexMap<DependencyId, BoxDependency>>,
    connections: RwLock<Arena<Connection>>,
    module_graph_modules: RwLock<Arena<ModuleGraphModule>>,
}
impl MemoryManager {
    pub fn new() -> Self {
        Self {
            module_caches: Default::default(),
            dependencies: Default::default(),
            connections: Default::default(),
            module_graph_modules: Default::default(),
        }
    }
}
// don't expose mutable borrow of arena's item
impl MemoryManager {
    pub fn alloc_module(&self, module: ReadonlyModule) -> ModuleId {
        let id = module.identifier();
        self.module_caches.insert(id, module);

        id
    }
    pub fn module_by_id(&self, id: ModuleId) -> ReadonlyModule {
        let module = self.module_caches.get(&id).unwrap().clone();
        module
    }
    pub fn alloc_dependency(&self, dep: BoxDependency) -> DependencyId {
        let dep_id = dep.id();
        self.dependencies.write().unwrap().insert(dep_id, dep);
        dep_id
    }
    pub fn dependency_by_id(&self, id: DependencyId) -> BoxDependency {
        self.dependencies
            .read()
            .unwrap()
            .get(&id)
            .expect("get dependency failed")
            .clone()
    }
    pub fn alloc_connection(&self, connection: Connection) -> ConnectionId {
        self.connections.write().unwrap().insert(connection)
    }
    pub fn connection_by_id(&self, connection_id: ConnectionId) -> Connection {
        self.connections.read().unwrap()[connection_id].clone()
    }
    pub fn alloc_module_graph_module(&self, mgm: ModuleGraphModule) -> ModuleGraphModuleId {
        self.module_graph_modules.write().unwrap().insert(mgm)
    }
    pub fn module_graph_module_by_id(&self, mgm_id: ModuleGraphModuleId) -> ModuleGraphModule {
        self.module_graph_modules.read().unwrap()[mgm_id].clone()
    }
}
