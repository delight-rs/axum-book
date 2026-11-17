use ch04_extractors::{AppState, app};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("ch04_extractors=debug")),
        )
        .init();

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "ch04-dev-secret".to_owned());
    let state = AppState::new(secret);

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on http://{addr}");
    tracing::info!("Dev token: POST /api/v1/dev-token");

    axum::serve(listener, app(state)).await.unwrap();
}
