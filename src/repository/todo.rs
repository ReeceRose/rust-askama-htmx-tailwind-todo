use anyhow::Result;
use sqlx::{Pool, Sqlite};
use tracing::error;

use crate::{error::TodoError, models::todo::Todo};

#[derive(Clone)]
pub struct TodoRepository {
    database: Pool<Sqlite>,
}

pub trait TodoRepositoryTrait {
    fn new(database: Pool<Sqlite>) -> Self;
    async fn all(&self) -> Result<Vec<Todo>, TodoError>;
    async fn get(&self, id: String) -> Result<Todo, TodoError>;
    async fn create(&mut self, todo: &Todo) -> Result<Todo, TodoError>;
    async fn update(&self, todo: &Todo) -> Result<bool, TodoError>; // TODO: update return to return Todo
    async fn delete(&self, id: String) -> Result<bool, TodoError>;
}

impl TodoRepositoryTrait for TodoRepository {
    fn new(database: Pool<Sqlite>) -> Self {
        TodoRepository { database }
    }

    async fn all(&self) -> Result<Vec<Todo>, TodoError> {
        let result = sqlx::query_as!(
            Todo,
            r#"
                SELECT id, created, updated, text, completed
                FROM todos
                ORDER BY completed, updated DESC
            "#
        )
        .fetch_all(&self.database)
        .await;

        match result {
            Ok(todos) => Ok(todos),
            Err(err) => {
                error!("Failed to get all todos from database: {}", err);
                Err(TodoError::FailedToGet)
            }
        }
    }

    async fn get(&self, id: String) -> Result<Todo, TodoError> {
        let result = sqlx::query_as!(
            Todo,
            r#"
                SELECT id, created, updated, text, completed
                FROM todos
                WHERE id = ?1
            "#,
            id
        )
        .fetch_optional(&self.database)
        .await;

        match result {
            Ok(result) => match result {
                Some(todo) => Ok(todo),
                None => Err(TodoError::NotFound),
            },
            Err(err) => {
                error!("Failed to get todo ({}) from database: {}", id, err);
                Err(TodoError::FailedToGet)
            }
        }
    }

    async fn create(&mut self, todo: &Todo) -> Result<Todo, TodoError> {
        let result = sqlx::query!(
            r#"
                INSERT INTO todos(id, created, updated, text, completed)
                VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            todo.id,
            todo.created,
            todo.updated,
            todo.text,
            todo.completed
        )
        .execute(&self.database)
        .await;

        match result {
            Ok(_result) => Ok(todo.clone()),
            Err(err) => {
                error!("Failed to create todo ({}) in database: {}", todo.text, err);
                Err(TodoError::FailedToGet)
            }
        }
    }

    async fn update(&self, todo: &Todo) -> Result<bool, TodoError> {
        let result = sqlx::query!(
            r#"
                UPDATE todos SET updated = ?1, text = ?2, completed = ?3
                WHERE id = $4
            "#,
            todo.updated,
            todo.text,
            todo.completed,
            todo.id
        )
        .execute(&self.database)
        .await;
        match result {
            Ok(_result) => Ok(true),
            Err(err) => {
                error!("Failed to update todo ({}) in database: {}", todo.id, err);
                Err(TodoError::FailedToUpdate)
            }
        }
    }

    async fn delete(&self, id: String) -> Result<bool, TodoError> {
        let result = sqlx::query!(
            r#"
                DELETE FROM todos
                WHERE id = $1
            "#,
            id
        )
        .execute(&self.database)
        .await;
        match result {
            Ok(_result) => Ok(true),
            Err(err) => {
                error!("Failed to delete todo ({}) from database: {}", id, err);
                Err(TodoError::FailedToUpdate)
            }
        }
    }
}
