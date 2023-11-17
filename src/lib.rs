use crate::routes::index::get_index;

mod error;
mod routes;
mod templates;

use axum::{routing::get, Router};

use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn app() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(get_index))
        .layer(TraceLayer::new_for_http())
}

pub async fn run() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_axum_tailwind_htmx_todo=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
