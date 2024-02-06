use crate::service::todo::TodoServiceImpl;

use axum::routing::{delete, get, Router};

use super::todo::{delete_todo, get_todos, post_todo, toggle_todo};

pub fn create_json_routes() -> Router<TodoServiceImpl> {
    return Router::new()
        .route("/todo", get(get_todos).post(post_todo))
        .route("/todo/:id", delete(delete_todo).patch(toggle_todo));
}
