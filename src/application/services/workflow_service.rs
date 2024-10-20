// src/application/services/workflow_service.rs

use crate::application::use_cases::create_workflow::create_workflow;
use crate::domain::entities::step::Step;
use crate::domain::entities::workflow::Workflow;
use crate::interfaces::repositories::workflowrepository::WorkflowRepository;

pub struct WorkflowService {
    repository: Box<dyn WorkflowRepository>,
}

impl WorkflowService {
    pub fn new(repository: Box<dyn WorkflowRepository>) -> Self {
        Self { repository }
    }

    pub fn create_workflow(
        &self,
        name: String,
        description: Option<String>,
        steps: Vec<Step>,
    ) -> Result<Workflow, String> {
        create_workflow(self.repository.as_ref(), name, description, steps)
    }

    pub fn list_workflows(&self) -> Result<Vec<Workflow>, String> {
        self.repository.find_all()
    }

    // Add more methods as needed for other use cases
}