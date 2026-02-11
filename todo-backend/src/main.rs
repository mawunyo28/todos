use std::env;

use dotenvy::dotenv;
use rocket::{State, http::Status, serde::json::Json};
use sqlx::{Pool, Postgres};
use todo_backend::{ClientData, Todo, db};

use uuid::Uuid;
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/todo", format = "json", data = "<payload>")]
async fn create_item(
    db: &State<Pool<Postgres>>,
    payload: Json<ClientData>,
) -> Result<Json<ClientData>, Status> {
    let ClientData {
        id,
        decription,
        is_done,
    } = payload.inner();
    let new_id = Uuid::new_v4();
    // let query = format!("", Uuid::new_v4(), payload.description, payload.is_done);

    let todo: Todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todo (id,  description, is_done, date)
        VALUES ($1, $2, $3, NOW())
        RETURNING id, description, is_done, date
    "#,
        new_id,
        payload.description,
        payload.is_done
    )
    .fetch_one(db.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    Ok(Json(ClientData::from(todo)))
}

#[get("/todos")]
fn todos() -> &'static str {
    "These are your todos"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let dburl = env::var("DATABASE_URL").expect("Database url must be set");
    let pool = db::create_pool(&dburl).await;

    let _rocket = rocket::build()
        .manage(pool)
        .mount("/", routes![index, todos, create_item])
        .launch()
        .await?;

    Ok(())
}
