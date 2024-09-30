use crate::module::ModuleIdentifier;


#[derive(Debug)]
pub(crate) struct AddTask {
    pub(crate) original_module_identifier: Option<ModuleIdentifier>,
}