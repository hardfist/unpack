use crate::dependency::Dependency;

pub struct ModuleGraph {
    pub dependencies: Vec<Box<dyn Dependency>>
}
