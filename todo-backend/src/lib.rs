use chrono::{DateTime, Utc};
pub mod db;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientData {
    pub description: String,
    pub is_done: bool,
}

#[derive(Debug, Serialize)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub is_done: bool,
    pub date: DateTime<Utc>,
}

// impl From<Todo> for ClientData {
//     fn from(value: Todo) -> Self {
//         ClientData {
//             id: value.id.to_string(),
//             description: value.description,
//             is_done: value.is_done,
//         }
//     }
// }
