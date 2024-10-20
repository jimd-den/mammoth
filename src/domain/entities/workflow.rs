use crate::domain::entities::step::Step;

pub struct Workflow {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<Step>,
}

impl Workflow {
    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }

    pub fn remove_step(&mut self, step_id: usize) -> Result<(), String> {
        if step_id < self.steps.len() {
            self.steps.remove(step_id);
            Ok(())
        } else {
            Err("Invalid step ID".to_string())
        }
    }

    // ... other methods for workflow management and execution (to be added later)
}
