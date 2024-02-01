use crate::{error::AppError, service::todo::TodoServiceImpl, templates::index::GetIndexResponse};

use anyhow::Result;
use axum::routing::{delete, get, Router};

use super::todo::{delete_todo, get_todos, post_todo, toggle_todo};

pub async fn get_index() -> Result<GetIndexResponse, AppError> {
    Ok(GetIndexResponse)
}

pub fn create_htmx_routes() -> Router<TodoServiceImpl> {
    return Router::new()
        .route("/todo", get(get_todos).post(post_todo))
        .route("/todo/:id", delete(delete_todo).patch(toggle_todo));
}
