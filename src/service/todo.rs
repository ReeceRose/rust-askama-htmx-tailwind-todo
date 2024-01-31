use anyhow::Result;

use crate::{
    models::Todo,
    repository::todo::{Repo, TodoRepo},
};

#[derive(Clone)]
pub struct TodoServiceImpl {
    repository: TodoRepo,
}

// TODO: Refactor return types
pub trait TodoService {
    fn new(repository: TodoRepo) -> Self; // TODO: borrow?
    async fn all(&self) -> Result<Vec<Todo>, String>;
    async fn get(&self, id: String) -> Result<Todo, String>;
    async fn create(&mut self, text: String) -> Result<Todo, String>;
    async fn update(&self, todo: &Todo) -> Result<Todo, String>;
    async fn delete(&self, id: String) -> Result<bool, String>;
}

impl TodoService for TodoServiceImpl {
    fn new(repository: TodoRepo) -> Self {
        Self { repository }
    }

    async fn all(&self) -> Result<Vec<Todo>, String> {
        let result = self.repository.all();
        if let Ok(todos) = result {
            return Ok(todos);
        }
        return Err("Failed to get todos".to_string());
    }

    async fn get(&self, id: String) -> Result<Todo, String> {
        let result = self.repository.get(id);
        if let Ok(todo) = result {
            return Ok(todo);
        }
        return Err("failed to get todo".to_string());
    }

    async fn create(&mut self, text: String) -> Result<Todo, String> {
        let todo = Todo::new(text);
        let result = self.repository.create(&todo);
        if let Ok(todo) = result {
            return Ok(todo);
        }
        return Err("failed to create todo".to_string());
    }

    async fn update(&self, todo: &Todo) -> Result<Todo, String> {
        let result = self.repository.update(todo);
        if let Ok(_result) = result {
            return Ok(todo.to_owned());
        }
        return Err("failed to update todo".to_string());
    }

    async fn delete(&self, id: String) -> Result<bool, String> {
        let result = self.repository.delete(id).unwrap();
        return Ok(result);
    }
}
