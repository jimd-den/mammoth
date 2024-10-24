use crate::{
    entities::workflow::Workflow,
    frameworks_drivers::gateways::ifworkflowrepository::IfWorkflowRepository,
};

pub struct CreateNewWorkFlow {
    repo: dyn IfWorkflowRepository,
}

impl CreateNewWorkFlow {
    pub fn execute(&mut self, _workflow: Workflow) -> Result<(), String> {
        self.repo.create(_workflow);
        Ok(())
    }
}
