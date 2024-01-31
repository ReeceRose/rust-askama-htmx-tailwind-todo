use crate::{
    error::AppError,
    models::TodoRequest,
    service::todo::{TodoService, TodoServiceImpl},
    templates::{EmptyResponse, ListTodoResponse},
    utils::get_timestamp,
};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    Form,
};

pub async fn get_todos(
    State(todo_service): State<TodoServiceImpl>,
) -> Result<ListTodoResponse, AppError> {
    let todos = todo_service.all().await.unwrap();
    Ok(ListTodoResponse { todos })
}

pub async fn post_todo(
    State(mut todo_service): State<TodoServiceImpl>,
    Form(todo): Form<TodoRequest>,
) -> Result<ListTodoResponse, AppError> {
    // let mut state = shared_state.write().unwrap();
    todo_service.create(todo.text).await;
    let todos = todo_service.all().await.unwrap();
    Ok(ListTodoResponse { todos })
}

pub async fn delete_todo(
    Path(id): Path<String>,
    State(todo_service): State<TodoServiceImpl>,
) -> Result<EmptyResponse, AppError> {
    todo_service.delete(id).await;
    Ok(EmptyResponse {})
}

pub async fn toggle_todo(
    Path(id): Path<String>,
    State(todo_service): State<TodoServiceImpl>,
) -> Result<EmptyResponse, AppError> {
    let mut todo = todo_service.get(id).await.unwrap();
    todo.completed = !todo.completed;
    todo.updated = get_timestamp();
    todo_service.update(&todo).await.unwrap();

    Ok(EmptyResponse {})
}
