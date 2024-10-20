// src/presentation/cli.rs

use crate::application::services::workflow_service::WorkflowService;
use crate::domain::entities::step::{Step, StepType};
use crate::domain::entities::form::Form;
use crate::domain::entities::checklist::Checklist;
use crate::domain::entities::routine::Routine;
use std::io::{self, Write};

pub struct CliPresentation {
    workflow_service: WorkflowService,
}

impl CliPresentation {
    pub fn new(workflow_service: WorkflowService) -> Self {
        Self { workflow_service }
    }

    pub fn run(&self) {
        println!("Welcome to MAMMOTH Workflow Manager!");
        loop {
            match self.show_main_menu() {
                1 => self.create_workflow(),
                2 => self.list_workflows(),
                3 => break,
                _ => println!("Invalid option. Please try again."),
            }
        }
        println!("Thank you for using MAMMOTH Workflow Manager!");
    }

    fn show_main_menu(&self) -> u32 {
        println!("\nMain Menu:");
        println!("1. Create a new workflow");
        println!("2. List all workflows");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        self.read_number()
    }

    fn create_workflow(&self) {
        println!("\nCreating a new workflow");
        let name = self.prompt("Enter workflow name: ");
        let description = self.prompt("Enter workflow description (optional): ");
        let description = if description.is_empty() { None } else { Some(description) };

        let mut steps = Vec::new();
        loop {
            match self.show_step_menu() {
                1 => steps.push(self.create_form_step()),
                2 => steps.push(self.create_checklist_step()),
                3 => steps.push(self.create_routine_step()),
                4 => break,
                _ => println!("Invalid option. Please try again."),
            }
        }

        match self.workflow_service.create_workflow(name, description, steps) {
            Ok(workflow) => println!("Workflow '{}' created successfully!", workflow.name),
            Err(e) => println!("Error creating workflow: {}", e),
        }
    }

    fn show_step_menu(&self) -> u32 {
        println!("\nAdd a step:");
        println!("1. Form");
        println!("2. Checklist");
        println!("3. Routine");
        println!("4. Finish adding steps");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        self.read_number()
    }

    fn create_form_step(&self) -> Step {
        let name = self.prompt("Enter form name: ");
        Step {
            step_type: StepType::Form(Form {
                name,
                fields: Vec::new(), // For simplicity, we're not adding fields in this example
            }),
            dependencies: Vec::new(),
        }
    }

    fn create_checklist_step(&self) -> Step {
        let name = self.prompt("Enter checklist name: ");
        Step {
            step_type: StepType::Checklist(Checklist {
                name,
                items: Vec::new(), // For simplicity, we're not adding items in this example
            }),
            dependencies: Vec::new(),
        }
    }

    fn create_routine_step(&self) -> Step {
        let name = self.prompt("Enter routine name: ");
        Step {
            step_type: StepType::Routine(Routine {
                name,
                action: String::new(), // For simplicity, we're not defining the action in this example
            }),
            dependencies: Vec::new(),
        }
    }

    fn list_workflows(&self) {
        match self.workflow_service.list_workflows() {
            Ok(workflows) => {
                println!("\nAll Workflows:");
                for (i, workflow) in workflows.iter().enumerate() {
                    println!("{}. {}", i + 1, workflow.name);
                }
            }
            Err(e) => println!("Error retrieving workflows: {}", e),
        }
    }

    fn prompt(&self, message: &str) -> String {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }

    fn read_number(&self) -> u32 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().unwrap_or(0)
    }
}