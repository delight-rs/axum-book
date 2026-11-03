use axum::{Router, routing::get};

pub fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
}

async fn root() -> &'static str {
    "Hello, world"
}

async fn health() -> &'static str {
    "ok"
}
