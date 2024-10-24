use crate::{
    application::workflow::create_new_workflow_uc::CreateNewWorkFlow, entities::workflow::Workflow,
};

pub struct WorkflowBuilder {
    create_workflow_uc: CreateNewWorkFlow,
}

impl WorkflowBuilder {
    pub fn create_new(&mut self, name: String, date_now: String) -> Result<(), String> {
        let workflow = Workflow {
            id: 0,
            name,
            steps: Vec::new(),
            date_created: date_now,
            date_updated: "null".to_string(),
        };
        self.create_workflow_uc.execute(workflow)
    }
}
