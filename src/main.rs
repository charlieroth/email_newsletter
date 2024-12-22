use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/health", get(health_handler));
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

async fn health_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}
