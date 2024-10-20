use std::collections::HashMap;
use crate::domain::entities::workflow::Workflow;

pub struct Routine {
    pub name: String,
    pub action: Option<fn(data: &HashMap<String, String>)>, // Function to execute
    pub workflow: Option<Box<Workflow>>, // Sub-workflow to execute
}
