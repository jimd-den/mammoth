// src/main.rs

mod application;
mod domain;
mod frameworks;
mod interfaces;

use crate::application::use_cases::create_workflow::create_workflow;
use crate::domain::entities::form::Form;
use crate::domain::entities::step::{Step, StepType};
use crate::frameworks::repositories::workflow_sqlite_repo::SqliteWorkflowRepository;
use crate::interfaces::repositories::workflowrepository::WorkflowRepository;
use std::io;

// Service struct to handle workflow operations
struct WorkflowService {
    repository: Box<dyn WorkflowRepository>,
}

impl WorkflowService {
    fn new(repository: Box<dyn WorkflowRepository>) -> Self {
        WorkflowService { repository }
    }

    // In src/main.rs

    fn create_workflow(
        &self,
        name: String,
        description: Option<String>,
        steps: Vec<Step>,
    ) -> Result<(), String> {
        match create_workflow(self.repository.as_ref(), name, description, steps) {
            // Corrected line
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

// CLI presentation struct to handle user interaction
struct CliPresentation {
    service: WorkflowService,
}

impl CliPresentation {
    fn new(service: WorkflowService) -> Self {
        CliPresentation { service }
    }

    fn run(&self) {
        loop {
            println!("Enter command (create, exit):");
            let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");
            let command = command.trim();

            match command {
                "create" => {
                    // Get workflow name from user
                    println!("Enter workflow name:");
                    let mut name = String::new();
                    io::stdin()
                        .read_line(&mut name)
                        .expect("Failed to read line");
                    let name = name.trim().to_string();

                    // Get workflow description from user (optional)
                    println!("Enter workflow description (optional):");
                    let mut description = String::new();
                    io::stdin()
                        .read_line(&mut description)
                        .expect("Failed to read line");
                    let description = if description.trim().is_empty() {
                        None
                    } else {
                        Some(description.trim().to_string())
                    };

                    // For now, create a simple workflow with a single empty form step
                    let steps = vec![Step {
                        step_type: StepType::Form(Form {
                            name: "Empty Form".to_string(),
                            fields: vec![],
                        }),
                        dependencies: vec![],
                    }];

                    // Call the create_workflow service method
                    match self.service.create_workflow(name, description, steps) {
                        Ok(_) => {
                            println!("Workflow created successfully!");
                        }
                        Err(error) => {
                            println!("Error creating workflow: {}", error);
                        }
                    }
                }
                "exit" => break,
                _ => println!("Invalid command"),
            }
        }
    }
}

fn main() {
    // Initialize the SQLite repository
    let repository = Box::new(SqliteWorkflowRepository::new("workflows.db").unwrap());

    // Create the workflow service
    let service = WorkflowService::new(repository);

    // Create the CLI presentation
    let cli = CliPresentation::new(service);

    // Run the CLI
    cli.run();
}
