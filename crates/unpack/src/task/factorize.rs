use std::mem;
use std::sync::Arc;

use derive_new::new;

use crate::compiler::CompilerOptions;
use crate::dependency::BoxDependency;
use crate::errors::UnpackDiagnostic;
use crate::module::ModuleId;
use crate::normal_module_factory::{ModuleFactoryCreateData, NormalModuleFactory};
use crate::task::Task;

use super::{AddTask, MakeTaskContext};
#[derive(Debug)]
pub(crate) struct FactorizeTask {
    pub(crate) module_dependency: BoxDependency,
    pub(crate) module_factory: Arc<NormalModuleFactory>,
    pub(crate) origin_module_id: Option<ModuleId>,
    pub(crate) options: Arc<CompilerOptions>,
}
#[derive(Debug, new)]
pub(crate) struct FactorizeTaskResult {
    pub(crate) diagnostics: Vec<UnpackDiagnostic>
}

impl Task<MakeTaskContext> for FactorizeTask {
    fn run(self: Box<Self>, task_context: &mut MakeTaskContext) -> super::TaskResult<MakeTaskContext> {
        let dependency = self.module_dependency;
        let factorize_result = FactorizeTaskResult::new(vec![]);
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
                Ok(vec![

                ])
            },
            Err(err) => {
                let mut diagnostics = mem::take(&mut create_data.diagnostics);
                diagnostics.push(err.into());
                task_context.artifact.diagnostics.extend(diagnostics);
                Ok(vec![
                    Box::new(
                        AddTask{
                            original_module_identifier: None
                        }
                    )
                ])
            }
        }
    }
}