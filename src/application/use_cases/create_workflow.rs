use crate::domain::entities::step::{Step, StepType};
use crate::domain::entities::workflow::Workflow;
use crate::interfaces::repositories::workflowrepository::WorkflowRepository;

pub fn create_workflow(
    repo: &dyn WorkflowRepository,
    name: String,
    description: Option<String>,
    steps: Vec<Step>,
) -> Result<Workflow, String> {
    // 1. Validate workflow data (e.g., name is not empty, steps are valid)
    if name.is_empty() {
        return Err("Workflow name cannot be empty".to_string());
    }
    // ... (other validation rules you might want to add)

    // 2. Create the Workflow entity
    let workflow = Workflow {
        name,
        description,
        steps,
    };

    // 3. Use the repository to save the workflow
    repo.create(workflow)
}
