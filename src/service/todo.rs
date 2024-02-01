use anyhow::Result;

use crate::{
    error::TodoError,
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
    async fn all(&self) -> Result<Vec<Todo>, TodoError>;
    async fn get(&self, id: String) -> Result<Todo, TodoError>;
    async fn create(&mut self, text: String) -> Result<Todo, TodoError>;
    async fn update(&self, todo: &Todo) -> Result<Todo, TodoError>;
    async fn delete(&self, id: String) -> Result<bool, TodoError>;
}

// TODO: add some more logging

impl TodoService for TodoServiceImpl {
    fn new(repository: TodoRepo) -> Self {
        Self { repository }
    }

    async fn all(&self) -> Result<Vec<Todo>, TodoError> {
        let result = self.repository.all();
        match result {
            Ok(todos) => Ok(todos),
            Err(_err) => Err(TodoError::FailedToGet),
        }
    }

    async fn get(&self, id: String) -> Result<Todo, TodoError> {
        let result = self.repository.get(id);
        match result {
            Ok(todo) => Ok(todo),
            Err(_err) => Err(TodoError::FailedToGet),
        }
    }

    async fn create(&mut self, text: String) -> Result<Todo, TodoError> {
        let todo = Todo::new(text);
        let result = self.repository.create(&todo);
        match result {
            Ok(todo) => Ok(todo),
            Err(_err) => Err(TodoError::FailedToCreate),
        }
    }

    async fn update(&self, todo: &Todo) -> Result<Todo, TodoError> {
        let result = self.repository.update(todo);
        match result {
            Ok(_updated) => Ok(todo.clone()),
            Err(_err) => Err(TodoError::FailedToUpdate),
        }
    }

    async fn delete(&self, id: String) -> Result<bool, TodoError> {
        let result = self.repository.delete(id);
        match result {
            Ok(deleted) => Ok(deleted),
            Err(_err) => Err(TodoError::FailedToDelete),
        }
    }
}
