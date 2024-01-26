use crate::{
    error::AppError,
    models::{SharedState, Todo, TodoRequest},
    templates::{DeleteTodoResponse, ListTodoResponse},
};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    Form,
};

pub async fn get_todos(
    State(shared_state): State<SharedState>,
) -> Result<ListTodoResponse, AppError> {
    let state = shared_state.write().unwrap();
    Ok(ListTodoResponse {
        todos: state.todos.to_vec(),
    })
}

pub async fn post_todo(
    State(shared_state): State<SharedState>,
    Form(todo): Form<TodoRequest>,
) -> Result<ListTodoResponse, AppError> {
    let mut state = shared_state.write().unwrap();
    state.todos.push(Todo::new(todo.text));
    Ok(ListTodoResponse {
        todos: state.todos.to_vec(),
    })
}

pub async fn delete_todo(
    Path(id): Path<String>,
    State(shared_state): State<SharedState>,
) -> Result<DeleteTodoResponse, AppError> {
    let mut state = shared_state.write().unwrap();
    let index = state
        .todos
        .iter()
        .position(|todo| todo.id.to_string() == id);
    match index {
        Some(index) => {
            state.todos.remove(index);
        }
        None => (),
    };
    Ok(DeleteTodoResponse {})
}
