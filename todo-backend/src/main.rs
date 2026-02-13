use std::env;

use dotenvy::dotenv;
use todo_backend::{
    db,
    routes::{create_item, todos},
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
