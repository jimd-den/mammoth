use crate::entities::workflow::Workflow;

pub trait IfWorkflowRepository: Send {
    fn create(&mut self, workflow: Workflow) -> Result<(), String>;
}
