use crate::frameworks_drivers::controllers::workflow_builder::WorkflowBuilder;

pub struct CreateWorkflowPresenter {
    pub controller: WorkflowBuilder,
}

pub struct CreateWorkflowViewModel {
    name: String,
    date: String,
}

impl CreateWorkflowPresenter {
    pub fn validate_input(
        create_workflow_viewmodel: CreateWorkflowViewModel,
    ) -> Result<CreateWorkflowViewModel, String> {
        //do we even need to validate input?
        Ok(create_workflow_viewmodel)
    }

    pub fn call(&mut self, name: String, date: String) -> Result<(), String> {
        let workflow = CreateWorkflowViewModel { name, date };
        let validated_workflow = Self::validate_input(workflow);

        match validated_workflow {
            Ok(workflow) => self.controller.create_new(workflow.name, workflow.date),
            _ => return Err(String::from("Error")),
        }
    }
}
