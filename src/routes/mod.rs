use axum::{
    routing::{delete, get},
    Router,
};

use crate::{service::todo::TodoServiceImpl, templates::index::GetIndexResponse};

use self::todo::{delete_todo, get_todos, post_todo, toggle_todo};

pub mod todo;

pub async fn get_index() -> GetIndexResponse {
    GetIndexResponse {}
}

pub fn create_routes() -> Router<TodoServiceImpl> {
    return Router::new()
        .route("/todo", get(get_todos).post(post_todo))
        .route("/todo/:id", delete(delete_todo).patch(toggle_todo));
}
