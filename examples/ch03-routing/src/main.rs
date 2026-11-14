use ch03_routing::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("ch03_routing=debug")),
        )
        .init();

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on http://{addr}");

    axum::serve(listener, app()).await.unwrap();
}
