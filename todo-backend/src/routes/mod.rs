use rocket::{State, get, http::Status, post, serde::json::Json};
use tokio_postgres::Client;

use crate::{ClientData, Todo};

#[post("/todo", format = "json", data = "<payload>")]
pub async fn create_item(db: &State<Client>, payload: Json<ClientData>) -> Result<Json<Todo>, Status> {
    let row = db.query_one("INSERT INTO todo (description, is_done) VALUES ($1, $2) RETURNING id, description, is_done, date", &[&payload.description, &payload.is_done])
        .await
    .expect("What is this");

    let todo = Todo {
        id: row.get(0),
        description: row.get(1),
        is_done: row.get(2),
        date: row.get(3),
    };

    Ok(Json(todo))
}

#[get("/todos")]
pub async fn todos(db: &State<Client>) -> Result<String, Status> {
    let rows = db
        .query("SELECT * FROM todo ORDER BY date DESC", &[])
        .await
        .expect("Unable to query /todos");

    let mut todos = Vec::new();

    for row in rows {
        let todo = Todo {
            id: row.get(0),
            description: row.get(1),
            is_done: row.get(2),
            date: row.get(3),
        };

        todos.push(todo);
    }

    Ok(todos.iter().map(|todo| todo.to_string()).collect())
}
