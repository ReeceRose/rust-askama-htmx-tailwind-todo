use anyhow::Result;

use crate::{
    error::TodoError,
    models::todo::Todo,
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
        let result = self.repository.all().await;
        match result {
            Ok(todos) => Ok(todos),
            Err(_err) => Err(TodoError::FailedToGet),
        }
    }

    async fn get(&self, id: String) -> Result<Todo, TodoError> {
        let result = self.repository.get(id).await;
        match result {
            Ok(todo) => Ok(todo),
            Err(_err) => Err(TodoError::FailedToGet),
        }
    }

    async fn create(&mut self, text: String) -> Result<Todo, TodoError> {
        if text.is_empty() {
            return Err(TodoError::EmptyTodo);
        }

        let todo = Todo::new(text);
        let result = self.repository.create(&todo).await;
        match result {
            Ok(todo) => Ok(todo),
            Err(_err) => Err(TodoError::FailedToCreate),
        }
    }

    async fn update(&self, todo: &Todo) -> Result<Todo, TodoError> {
        let result = self.repository.update(todo).await;
        match result {
            Ok(_updated) => Ok(todo.clone()),
            Err(_err) => Err(TodoError::FailedToUpdate),
        }
    }

    async fn delete(&self, id: String) -> Result<bool, TodoError> {
        let result = self.repository.delete(id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(_err) => Err(TodoError::FailedToDelete),
        }
    }
}
