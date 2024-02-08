use crate::utils::get_timestamp;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub created: i64,
    pub updated: i64,
    pub text: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(text: String) -> Self {
        let timestamp = get_timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            created: timestamp,
            updated: timestamp,
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
