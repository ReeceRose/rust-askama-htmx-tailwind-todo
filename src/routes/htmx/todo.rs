use crate::{
    models::todo::TodoRequest,
    service::todo::{TodoService, TodoServiceImpl},
    templates::{
        index::EmptyResponse,
        todo::{
            CreateTodoErrorResponse, DeleteTodoErrorResponse, ListTodoErrorResponse,
            ListTodoResponse, ToggleTodoErrorResponse,
        },
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
        Err(err) => (
            StatusCode::OK,
            [("HX-Retarget", "#submit-error"), ("HX-Reswap", "innerHTML")],
            CreateTodoErrorResponse {
                error: err.to_string(),
            },
        )
            .into_response(),
    }
}

pub async fn delete_todo(
    Path(id): Path<String>,
    State(todo_service): State<TodoServiceImpl>,
) -> impl IntoResponse {
    let result = todo_service.delete(id).await;

    match result {
        Ok(_result) => (EmptyResponse {}).into_response(),
        Err(err) => (
            StatusCode::OK,
            [("HX-Retarget", "#delete-error"), ("HX-Reswap", "innerHTML")],
            DeleteTodoErrorResponse {
                error: err.to_string(),
            },
        )
            .into_response(),
    }
}

pub async fn toggle_todo(
    Path(id): Path<String>,
    State(todo_service): State<TodoServiceImpl>,
) -> impl IntoResponse {
    let result = todo_service.get(id).await;

    match result {
        Ok(mut todo) => {
            todo.completed = !todo.completed;
            todo.updated = get_timestamp();
            let result = todo_service.update(&todo).await;
            match result {
                Ok(_a) => (EmptyResponse {}).into_response(),
                Err(err) => (
                    StatusCode::OK,
                    [("HX-Retarget", "#toggle-error"), ("HX-Reswap", "innerHTML")],
                    ToggleTodoErrorResponse {
                        error: err.to_string(),
                    },
                )
                    .into_response(),
            }
        }
        Err(err) => (ToggleTodoErrorResponse {
            error: err.to_string(),
        })
        .into_response(),
    }
}
