use chrono::{DateTime, Utc};
pub mod db;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientData {
    pub id: String,
    pub description: String,
    pub is_done: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub description: String,
    pub is_done: bool,
    pub date: DateTime<Utc>,
}

impl From<Todo> for ClientData {
    fn from(value: Todo) -> Self {
        ClientData {
            id: value.id.to_string(),
            description: value.description,
            is_done: value.is_done,
        }
    }
}
