use crate::{
    models::{
        response::{BaseResponse, Json},
        todo::{ListTodosResponse, TodoRequest},
    },
    service::todo::{TodoService, TodoServiceTrait},
    templates::{
        index::EmptyResponse,
        todo::{
            CreateTodoErrorResponse, DeleteTodoErrorResponse, ListTodoErrorResponse,
            ListTodoResponse, ToggleTodoErrorResponse,
        },
    },
    utils::get_timestamp,
};

use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension,
};

pub async fn get_todos(
    Extension(htmx): Extension<bool>,
    State(todo_service): State<TodoService>,
) -> impl IntoResponse {
    let result = todo_service.all().await;
    if htmx {
        return match result {
            Ok(todos) => (ListTodoResponse { todos }).into_response(),
            Err(err) => (
                StatusCode::OK,
                [("HX-Retarget", "this"), ("HX-Reswap", "innerHTML")],
                ListTodoErrorResponse {
                    error: err.to_string(),
                },
            )
                .into_response(),
        };
    }

    return match result {
        Ok(todos) => Json(ListTodosResponse::success(todos)),
        Err(err) => Json(ListTodosResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            err.to_string(),
        )),
    }
    .into_response();
}

pub async fn post_todo(
    Extension(htmx): Extension<bool>,
    State(mut todo_service): State<TodoService>,
    Json(todo): Json<TodoRequest>,
) -> impl IntoResponse {
    let result = todo_service.create(todo.text).await;

    if htmx {
        return match result {
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
        };
    }

    return match result {
        Ok(_todo) => Json(BaseResponse::success()),
        Err(err) => Json(BaseResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            err.to_string(),
        )),
    }
    .into_response();
}

pub async fn delete_todo(
    Extension(htmx): Extension<bool>,
    Path(id): Path<String>,
    State(todo_service): State<TodoService>,
) -> impl IntoResponse {
    let result = todo_service.delete(id).await;

    if htmx {
        return match result {
            Ok(_result) => (EmptyResponse {}).into_response(),
            Err(err) => (
                StatusCode::OK,
                [("HX-Retarget", "#delete-error"), ("HX-Reswap", "innerHTML")],
                DeleteTodoErrorResponse {
                    error: err.to_string(),
                },
            )
                .into_response(),
        };
    }

    return match result {
        Ok(_result) => Json(BaseResponse::success()),
        Err(err) => Json(BaseResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            err.to_string(),
        )),
    }
    .into_response();
}

pub async fn toggle_todo(
    Extension(htmx): Extension<bool>,
    Path(id): Path<String>,
    State(todo_service): State<TodoService>,
) -> impl IntoResponse {
    let result = todo_service.get(id).await;

    match result {
        Ok(mut todo) => {
            todo.completed = !todo.completed;
            todo.updated = get_timestamp();
            let result = todo_service.update(&todo).await;

            if htmx {
                return match result {
                    Ok(_a) => (EmptyResponse {}).into_response(),
                    Err(err) => (
                        StatusCode::OK,
                        [("HX-Retarget", "#toggle-error"), ("HX-Reswap", "innerHTML")],
                        ToggleTodoErrorResponse {
                            error: err.to_string(),
                        },
                    )
                        .into_response(),
                };
            }
            return match result {
                Ok(_a) => Json(BaseResponse::success()),
                Err(err) => Json(BaseResponse::error(
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    err.to_string(),
                )),
            }
            .into_response();
        }
        Err(err) => {
            if htmx {
                return ToggleTodoErrorResponse {
                    error: err.to_string(),
                }
                .into_response();
            }
            return Json(BaseResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                err.to_string(),
            ))
            .into_response();
        }
    }
}
