use crate::{
    error::AppError,
    models::{SharedState, Todo, TodoRequest},
    templates::ListTodoResponse,
};

use anyhow::Result;
use axum::{extract::State, Form};

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
