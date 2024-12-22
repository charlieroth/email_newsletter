use crate::http::handlers::health::health_handler;
use anyhow::Context;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

mod handlers;
mod responses;

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

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    async fn spawn_app() {
        let http_server = HttpServer::new()
            .await
            .expect("Failed to create http server.");

        let _ = tokio::spawn(async move {
            http_server.run().await.expect("Failed to run http server.");
        });
    }

    #[tokio::test]
    async fn health_check_works() {
        spawn_app().await;

        let client = Client::new();
        let response = client
            .get("http://localhost:8000/health")
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(response.status().is_success());
    }
}
