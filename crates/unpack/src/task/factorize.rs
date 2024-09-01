use std::mem;
use std::sync::Arc;

use crate::compiler::CompilerOptions;
use crate::dependency::{BoxDependency};
use crate::module::ModuleId;
use crate::normal_module_factory::{ModuleFactoryCreateData, NormalModuleFactory};
use crate::task::Task;
#[derive(Debug)]
pub(crate) struct FactorizeTask {
    pub(crate) module_dependency: BoxDependency,
    pub(crate) module_factory: Arc<NormalModuleFactory>,
    pub(crate) origin_module_id: Option<ModuleId>,
    pub(crate) options: Arc<CompilerOptions>,
}

impl Task for FactorizeTask {
    fn run(self: Box<Self>) -> super::TaskResult {
        let dependency = self.module_dependency;
        let context = if let Some(context) = dependency.get_context() 
        {
            context.to_path_buf()
        } else {
            self.options.context.clone()
        };
        let mut create_data = ModuleFactoryCreateData {
            options: self.options.clone(),
            context,
            module_dependency: dependency,
            diagnostics: Default::default()
        };
        match self.module_factory.create(&mut create_data) {
            Ok(result) => {

            },
            Err(err) => {
                let mut diagnotics = mem::take(&mut create_data.diagnostics);
                diagnotics.push(err.into());
                dbg!(diagnotics);
            }
        }
        Ok(vec![])
    }
}