use crate::{
    models::TodoRequest,
    service::todo::{TodoService, TodoServiceImpl},
    templates::{
        index::EmptyResponse,
        todo::{ListTodoErrorResponse, ListTodoResponse},
    },
    utils::get_timestamp,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Form,
};

pub async fn get_todos(State(todo_service): State<TodoServiceImpl>) -> impl IntoResponse {
    let result = todo_service.all().await;

    match result {
        Ok(todos) => (ListTodoResponse { todos }).into_response(),
        Err(err) => (
            StatusCode::OK,
            [("HX-Retarget", "this"), ("HX-Reswap", "innerHTML")],
            ListTodoErrorResponse {
                error: err.to_string(),
            },
        )
            .into_response(),
    }
}

pub async fn post_todo(
    State(mut todo_service): State<TodoServiceImpl>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    let result = todo_service.create(todo.text).await;
    match result {
        Ok(_todo) => {
            let todos = todo_service.all().await.unwrap();
            ListTodoResponse { todos }.into_response()
        }
        Err(_err) => EmptyResponse {}.into_response(),
    }
}

pub async fn delete_todo(
    Path(id): Path<String>,
    State(todo_service): State<TodoServiceImpl>,
) -> impl IntoResponse {
    todo_service.delete(id).await;
    EmptyResponse {}
}

pub async fn toggle_todo(
    Path(id): Path<String>,
    State(todo_service): State<TodoServiceImpl>,
) -> impl IntoResponse {
    let mut todo = todo_service.get(id).await.unwrap();
    todo.completed = !todo.completed;
    todo.updated = get_timestamp();
    todo_service.update(&todo).await.unwrap();

    EmptyResponse {}
}
