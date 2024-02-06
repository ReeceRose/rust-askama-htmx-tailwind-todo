use std::sync::{Arc, RwLock};

use super::todo::Todo;

pub struct AppState {
    pub todos: Vec<Todo>, // pub todo_repo
}

impl Default for AppState {
    fn default() -> Self {
        Self { todos: vec![] }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;
