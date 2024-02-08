mod error;
mod models;
mod repository;
mod routes;
mod service;
mod templates;
mod utils;

use axum::{routing::get, Router};
use sqlx::sqlite::SqlitePool;
use std::env::var;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use repository::todo::{Repo, TodoRepo};
use routes::{
    htmx::index::{create_htmx_routes, get_index},
    json::index::create_json_routes,
};
use service::todo::{TodoService, TodoServiceImpl};

pub async fn app() -> Result<Router, anyhow::Error> {
    // let database_url = env!("DATABASE_URL")
    let database_url = var("DATABASE_URL").unwrap_or("sqlite:todos.db".to_string());
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to open database connection");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to create database pool");

    let todo_repo = TodoRepo::new(pool);
    let todo_service = TodoServiceImpl::new(todo_repo);

    Ok(Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(get_index))
        .nest("/htmx-api", create_htmx_routes())
        .nest("/json-api", create_json_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(todo_service))
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
    axum::serve(listener, app().await.unwrap()).await.unwrap();
}
