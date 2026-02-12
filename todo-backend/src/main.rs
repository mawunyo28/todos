use std::env;

use dotenvy::dotenv;
use rocket::{State, http::Status, serde::json::Json};
use todo_backend::{ClientData, Todo, db};
use tokio_postgres::Client;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/todo", format = "json", data = "<payload>")]
async fn create_item(db: &State<Client>, payload: Json<ClientData>) -> Result<Json<Todo>, Status> {
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
fn todos() -> &'static str {
    "These are your todos"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let dburl = env::var("DATABASE_URL").expect("Database url must be set");
    let client = db::create_client(&dburl).await;

    let _rocket = rocket::build()
        .manage(client)
        .mount("/", routes![index, todos, create_item])
        .launch()
        .await?;

    Ok(())
}
