use super::todo::Todo;

pub struct AppState {
    pub todos: Vec<Todo>, // pub todo_repo
}

impl Default for AppState {
    fn default() -> Self {
        Self { todos: vec![] }
    }
}
