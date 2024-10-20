use crate::domain::entities::workflow::Workflow;

pub trait WorkflowRepository: Send + Sync {
    fn create(&self, workflow: Workflow) -> Result<Workflow, String>;
    fn update(&self, workflow: Workflow) -> Result<Workflow, String>;
    fn delete(&self, workflow_id: String) -> Result<(), String>;
    fn find_by_id(&self, workflow_id: String) -> Result<Option<Workflow>, String>;
    fn find_all(&self) -> Result<Vec<Workflow>, String>;
    // ... other methods as needed (e.g., find_by_name)
}
