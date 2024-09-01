use super::{MakeArtifact, MakeTaskContext, Task};

#[derive(Debug)]
pub(crate) struct AddTask {

}

impl Task<MakeTaskContext> for AddTask {
    fn run(self: Box<Self>, context: &mut MakeTaskContext) -> super::TaskResult<MakeTaskContext> {
        println!("add task");
        Ok(vec![])
    }
}