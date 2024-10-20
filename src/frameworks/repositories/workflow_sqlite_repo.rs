// src/interfaces/repositories/workflow_repository.rs

use crate::domain::entities::workflow::Workflow;
use crate::interfaces::repositories::workflowrepository::WorkflowRepository;
use rusqlite::{params, Connection, OptionalExtension, Result};
use std::sync::{Arc, Mutex};

pub struct SqliteWorkflowRepository {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteWorkflowRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Create the workflows table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS workflows (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT
            )",
            (),
        )?;

        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
        })
    }
}

impl WorkflowRepository for SqliteWorkflowRepository {
    fn create(&self, workflow: Workflow) -> Result<Workflow, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        conn.execute(
            "INSERT INTO workflows (name, description) VALUES (?1, ?2)",
            params![workflow.name, workflow.description],
        )
        .map_err(|e| format!("Failed to insert workflow: {}", e))?;

        // Get the last inserted row ID (which is the workflow ID)
        let id = conn.last_insert_rowid();

        // For simplicity, we're just returning the workflow with the generated ID here.
        // In a real application, you might want to fetch the created workflow from the
        // database to include any default values or timestamps set by the database.

        Ok(Workflow {
            name: workflow.name,
            description: workflow.description,
            steps: workflow.steps, // You'll need to handle steps serialization
        })
    }

    fn update(&self, workflow: Workflow) -> Result<Workflow, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        let id = 1; // Replace with the actual workflow ID

        conn.execute(
            "UPDATE workflows SET name = ?1, description = ?2 WHERE id = ?3",
            params![workflow.name, workflow.description, id],
        )
        .map_err(|e| format!("Failed to update workflow: {}", e))?;

        Ok(workflow) // Return the updated workflow
    }

    fn delete(&self, workflow_id: String) -> Result<(), String> {
        let conn = self
            .connection
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        let id = workflow_id
            .parse::<i64>()
            .map_err(|e| format!("Invalid workflow ID: {}", e))?;

        conn.execute("DELETE FROM workflows WHERE id = ?1", [id])
            .map_err(|e| format!("Failed to delete workflow: {}", e))?;

        Ok(())
    }

    fn find_by_id(&self, workflow_id: String) -> Result<Option<Workflow>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        let id = workflow_id
            .parse::<i64>()
            .map_err(|e| format!("Invalid workflow ID: {}", e))?;

        let mut stmt = conn
            .prepare("SELECT name, description FROM workflows WHERE id = ?1")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let workflow = stmt
            .query_row([id], |row| {
                Ok(Workflow {
                    name: row.get(0)?,
                    description: row.get(1)?,
                    steps: vec![], // You'll need to handle steps deserialization
                })
            })
            .optional()
            .map_err(|e| format!("Failed to find workflow: {}", e))?;

        Ok(workflow)
    }

    fn find_all(&self) -> Result<Vec<Workflow>, String> {
        let conn = self
            .connection
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        let mut stmt = conn
            .prepare("SELECT id, name, description FROM workflows")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let workflow_iter = stmt
            .query_map([], |row| {
                Ok(Workflow {
                    name: row.get(1)?,
                    description: row.get(2)?,
                    steps: vec![], // You'll need to handle steps deserialization
                })
            })
            .map_err(|e| format!("Failed to query workflows: {}", e))?;

        let workflows: Result<Vec<_>, _> = workflow_iter.collect();
        workflows.map_err(|e| format!("Failed to collect workflows: {}", e))
    }
}
