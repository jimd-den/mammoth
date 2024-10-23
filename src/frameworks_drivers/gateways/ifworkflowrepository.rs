use crate::entities::workflow::Workflow;

pub trait IfWorkflowRepository {
    fn create(&mut self, workflow: Workflow) -> Result<(), String>;
}
