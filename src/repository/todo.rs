use std::collections::HashMap;

use uuid::Uuid;

use anyhow::Result;

use crate::models::Todo;

pub enum TodoRepoError {
    NotFound,
}

pub struct TodoRepo {
    pub todos: HashMap<Uuid, Todo>,
}

trait Repo {
    fn new() -> Self;
    fn get(&self, id: &Uuid) -> Result<Todo, TodoRepoError>;
    fn create(&mut self, todo: Todo);
}

impl Repo for TodoRepo {
    fn get(&self, id: &Uuid) -> Result<Todo, TodoRepoError> {
        self.todos.get(id).cloned().ok_or(TodoRepoError::NotFound)
    }

    fn create(&mut self, todo: Todo) {
        self.todos.insert(todo.id, todo);
    }

    fn new() -> Self {
        TodoRepo {
            todos: HashMap::new(),
        }
    }
}
