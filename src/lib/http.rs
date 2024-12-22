use crate::http::handlers::health::health_handler;
use anyhow::Context;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

mod handlers;

pub struct HttpServer {
    pub router: Router,
    pub listener: TcpListener,
}

impl HttpServer {
    pub async fn new() -> anyhow::Result<Self> {
        let router = Router::new().route("/health", get(health_handler));
        let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.router)
            .await
            .context("received error from axum server")?;

        Ok(())
    }
}
