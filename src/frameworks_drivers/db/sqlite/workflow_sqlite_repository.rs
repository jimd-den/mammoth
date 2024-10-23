use crate::entities::workflow::Workflow;
use crate::entities::workflow::WorkflowStep;
use crate::frameworks_drivers::gateways::ifworkflowrepository::IfWorkflowRepository;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
pub struct WorkflowSqliteRepository {
    conn: Arc<Mutex<Connection>>,
}

impl WorkflowSqliteRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn execute_query(&self, query: &str, params: &[&dyn rusqlite::ToSql]) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(query, params)
            .map_err(|e| e.to_string())
            .map(|_| ())
    }
}

impl IfWorkflowRepository for WorkflowSqliteRepository {
    fn create(&mut self, workflow: Workflow) -> Result<(), String> {
        let query = "INSERT INTO workflows (name, date_created, date_updated) VALUES (?, ?, ?)";
        self.execute_query(
            query,
            &[
                &workflow.name,
                &workflow.date_created,
                &workflow.date_updated,
            ],
        )?;

        let workflow_id = self.conn.lock().unwrap().last_insert_rowid();

        for step in workflow.steps {
            let query = "INSERT INTO workflow_steps (workflow_id, seq) VALUES (?, ?)";
            self.execute_query(query, &[&workflow_id, &step.seq])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE workflows (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                date_created TEXT NOT NULL,
                date_updated TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE workflow_steps (
                id INTEGER PRIMARY KEY,
                workflow_id INTEGER,
                seq INTEGER,
                FOREIGN KEY(workflow_id) REFERENCES workflows(id)
            )",
            [],
        )
        .unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[test]
    fn test_create_workflow() {
        let conn = setup_test_db();
        let mut repo = WorkflowSqliteRepository::new(conn.clone());

        let workflow = Workflow {
            id: 0,
            name: "Test Workflow".to_string(),
            steps: vec![WorkflowStep { id: 0, seq: 1 }],
            date_created: "2023-05-01".to_string(),
            date_updated: "2023-05-01".to_string(),
        };

        assert!(repo.create(workflow).is_ok());

        let conn = conn.lock().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM workflows", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        let step_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM workflow_steps", [], |row| row.get(0))
            .unwrap();
        assert_eq!(step_count, 1);
    }
}
