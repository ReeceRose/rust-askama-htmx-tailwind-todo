mod repository;
mod routes;
mod service;

mod error;
mod models;
mod templates;
mod utils;

use axum::{
    routing::{delete, get},
    Router,
};

use models::SharedState;
use repository::todo::{Repo, TodoRepo};
use routes::htmx::{
    index::get_index,
    todo::{delete_todo, get_todos, post_todo, toggle_todo},
};
use service::todo::{TodoService, TodoServiceImpl};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn app() -> Router {
    let todo_repo = TodoRepo::new(SharedState::default());
    let todo_service = TodoServiceImpl::new(todo_repo);

    let htmx_routes = Router::new()
        .route("/todo", get(get_todos).post(post_todo))
        .route("/todo/:id", delete(delete_todo).patch(toggle_todo));

    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(get_index))
        .nest("/htmx-api", htmx_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(todo_service)
}

pub async fn run() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust-askama-htmx-tailwind-todo=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT").unwrap_or(String::from("3000"));

    let addr = format!("0.0.0.0:{}", port);
    info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}
