pub struct WorkflowStep {
    id: i64,
    seq: i32,
}

pub struct Workflow {
    id: i64,
    name: String,
    steps: Vec<WorkflowStep>,
    date_created: String,
    date_updated: String,
}
