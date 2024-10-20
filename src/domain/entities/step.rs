use crate::domain::entities::{form::Form, checklist::Checklist, routine::Routine};

pub enum StepType {
    Form(Form),
    Checklist(Checklist),
    Routine(Routine),
}

pub struct Step {
    pub step_type: StepType,
    pub dependencies: Vec<usize>, // IDs of steps that must be completed before this one
}
