use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::get_timestamp;

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    created: u128,
    updated: u128,
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

    pub fn complete(mut self) -> Self {
        let timestamp = get_timestamp();
        self.completed = true;
        self.updated = timestamp;
        self
    }
}

#[derive(Deserialize)]
pub struct TodoRequest {
    pub text: String,
}

// #[derive(Clone)]
pub struct AppState {
    pub todos: Vec<Todo>, // pub todo_repo
}

impl Default for AppState {
    fn default() -> Self {
        Self { todos: vec![] }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;
