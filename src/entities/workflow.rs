pub struct WorkflowStep {
    pub id: i64,
    pub seq: i32,
}

pub struct Workflow {
    pub id: i64,
    pub name: String,
    pub steps: Vec<WorkflowStep>,
    pub date_created: String,
    pub date_updated: String,
}
