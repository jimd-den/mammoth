// src/main.rs

mod application;
mod domain;
mod frameworks;
mod interfaces;
mod presentation;

use crate::application::services::workflow_service::WorkflowService;
use crate::frameworks::repositories::workflow_sqlite_repo::SqliteWorkflowRepository;
use crate::presentation::cli::CliPresentation;

fn main() {
    let repository = Box::new(SqliteWorkflowRepository::new("workflows.db").unwrap());
    let workflow_service = WorkflowService::new(repository);
    let cli = CliPresentation::new(workflow_service);
    cli.run();
}
