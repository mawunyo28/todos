pub mod middleware;
pub mod routes;

pub mod db;

use std::fmt::Display;

use chrono::{DateTime, Utc};

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

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Todo item {}: {}\nDone?: {}\nDate:{}",
            self.id, self.description, self.is_done, self.date
        )
    }
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
