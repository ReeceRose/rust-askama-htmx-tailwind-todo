use crate::{
    models::{
        response::BaseResponse,
        todo::{ListTodosResponse, TodoRequest},
    },
    service::todo::{TodoService, TodoServiceImpl},
    utils::get_timestamp,
};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn get_todos(State(todo_service): State<TodoServiceImpl>) -> Json<ListTodosResponse> {
    let result = todo_service.all().await;

    match result {
        Ok(todos) => Json(ListTodosResponse::success(todos)),
        Err(err) => Json(ListTodosResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            err.to_string(),
        )),
    }
}

pub async fn post_todo(
    State(mut todo_service): State<TodoServiceImpl>,
    Json(todo): Json<TodoRequest>,
) -> Json<BaseResponse> {
    let result = todo_service.create(todo.text).await;

    match result {
        Ok(_todo) => Json(BaseResponse::success()),
        Err(err) => Json(BaseResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            err.to_string(),
        )),
    }
}

pub async fn delete_todo(
    Path(id): Path<String>,
    State(todo_service): State<TodoServiceImpl>,
) -> Json<BaseResponse> {
    let result = todo_service.delete(id).await;

    match result {
        Ok(_result) => Json(BaseResponse::success()),
        Err(err) => Json(BaseResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            err.to_string(),
        )),
    }
}

pub async fn toggle_todo(
    Path(id): Path<String>,
    State(todo_service): State<TodoServiceImpl>,
) -> Json<BaseResponse> {
    let result = todo_service.get(id).await;

    match result {
        Ok(mut todo) => {
            todo.completed = !todo.completed;
            todo.updated = get_timestamp();
            let result = todo_service.update(&todo).await;
            match result {
                Ok(_a) => Json(BaseResponse::success()),
                Err(err) => Json(BaseResponse::error(
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    err.to_string(),
                )),
            }
        }
        Err(err) => Json(BaseResponse::error(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            err.to_string(),
        )),
    }
}
