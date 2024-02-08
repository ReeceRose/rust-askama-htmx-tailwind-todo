use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::{error::TodoError, models::todo::Todo};

// TODO: Refactor name
#[derive(Clone)]
pub struct TodoRepo {
    database: Pool<Sqlite>,
}

// TODO: Refactor name
pub trait Repo {
    fn new(database: Pool<Sqlite>) -> Self;
    async fn all(&self) -> Result<Vec<Todo>, TodoError>;
    async fn get(&self, id: String) -> Result<Todo, TodoError>;
    async fn create(&mut self, todo: &Todo) -> Result<Todo, TodoError>;
    async fn update(&self, todo: &Todo) -> Result<bool, TodoError>; // TODO: update return to return Todo
    async fn delete(&self, id: String) -> Result<bool, TodoError>;
}

impl Repo for TodoRepo {
    fn new(database: Pool<Sqlite>) -> Self {
        TodoRepo { database }
    }

    async fn all(&self) -> Result<Vec<Todo>, TodoError> {
        let result = sqlx::query_as!(
            Todo,
            r#"
                SELECT id, created, updated, text, completed
                FROM todos
                ORDER BY completed
            "#
        )
        .fetch_all(&self.database)
        .await;

        match result {
            Ok(todos) => Ok(todos),
            Err(_err) => Err(TodoError::FailedToGet),
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
            Err(_err) => Err(TodoError::FailedToGet),
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
            Err(_err) => Err(TodoError::FailedToGet),
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
            Err(_err) => Err(TodoError::FailedToUpdate),
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
            Err(_err) => Err(TodoError::FailedToUpdate),
        }
    }
}
