#[macro_use]
extern crate rocket;

mod application;
mod entities;
mod frameworks_drivers;

use application::workflow::create_new_workflow_uc::CreateNewWorkFlow;
use frameworks_drivers::controllers::workflow_builder::WorkflowBuilder;
use frameworks_drivers::db::sqlite::workflow_sqlite_repository::WorkflowSqliteRepository;
use frameworks_drivers::presenters::create_workflow_presenter::CreateWorkflowPresenter;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[derive(FromForm)]
struct WorkflowForm {
    name: String,
    date: String,
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Create Workflow</title>
        </head>
        <body>
            <h1>Create New Workflow</h1>
            <form action="/create_workflow" method="post">
                <label for="name">Workflow Name:</label>
                <input type="text" id="name" name="name" required><br><br>
                <label for="date">Date:</label>
                <input type="date" id="date" name="date" required><br><br>
                <input type="submit" value="Create Workflow">
            </form>
        </body>
        </html>
    "#,
    )
}

#[post("/create_workflow", data = "<form>")]
fn create_workflow(
    form: Form<WorkflowForm>,
    presenter: &rocket::State<Mutex<CreateWorkflowPresenter>>,
) -> String {
    let mut presenter = presenter.lock().unwrap();
    let result = presenter.call(form.name.clone(), form.date.clone());

    match result {
        Ok(_) => "Workflow created successfully!".to_string(),
        Err(e) => format!("Error creating workflow: {}", e),
    }
}

#[launch]
fn rocket() -> _ {
    // Set up database connection
    let db_connection = setup_test_db();
    // Set up dependencies
    let repo = WorkflowSqliteRepository::new(db_connection);
    let create_workflow_uc = CreateNewWorkFlow {
        repo: Box::new(repo),
    };
    let workflow_builder = WorkflowBuilder { create_workflow_uc };
    let presenter = Mutex::new(CreateWorkflowPresenter {
        controller: workflow_builder,
    });

    rocket::build()
        .mount("/", routes![index, create_workflow])
        .manage(presenter)
}

fn setup_test_db() -> Arc<Mutex<Connection>> {
    let conn = Connection::open("mammoth.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS workflows (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                date_created TEXT NOT NULL,
                date_updated TEXT NOT NULL
            )",
        [],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS workflow_steps (
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
