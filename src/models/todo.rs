use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::get_timestamp;

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    created: u128,
    pub updated: u128,
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(text: String) -> Self {
        let timestamp = get_timestamp();
        Self {
            created: timestamp,
            updated: timestamp,
            id: Uuid::new_v4(),
            text,
            completed: false,
        }
    }
}

#[derive(Deserialize)]
pub struct TodoRequest {
    pub text: String,
}

#[derive(Serialize)]
pub struct ListTodosResponse {
    pub todos: Vec<Todo>,
    pub success: bool,
    pub error: String,
    pub status_code: u16,
}

impl ListTodosResponse {
    pub fn success(todos: Vec<Todo>) -> Self {
        Self {
            status_code: StatusCode::OK.as_u16(),
            success: true,
            error: "".to_string(),
            todos,
        }
    }

    pub fn error(status_code: u16, error: String) -> Self {
        Self {
            status_code,
            success: false,
            error,
            todos: vec![],
        }
    }
}
