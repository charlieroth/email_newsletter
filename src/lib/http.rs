use std::sync::Arc;

use crate::http::handlers::{health::health_handler, subscriptions::subscriptions_handler};
use anyhow::Context;
use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod handlers;
mod responses;

#[derive(Debug, Clone)]
pub struct AppState<PG> {
    connection: Arc<PG>,
}

pub struct HttpServer {
    pub router: Router,
}

impl HttpServer {
    pub async fn new(pool: PgPool) -> anyhow::Result<Self> {
        let router = Router::new()
            .route("/health", get(health_handler))
            .route("/subscriptions", post(subscriptions_handler))
            .with_state(pool);

        Ok(Self { router })
    }

    pub async fn run(self, listener: TcpListener) -> anyhow::Result<()> {
        axum::serve(listener, self.router)
            .await
            .context("received error from axum server")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    async fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let http_server = HttpServer::new()
            .await
            .expect("Failed to create http server.");

        let _ = tokio::spawn(async move {
            http_server
                .run(listener)
                .await
                .expect("Failed to run http server.");
        });

        format!("http://127.0.0.1:{}", port)
    }

    #[tokio::test]
    async fn health_check_works() {
        let app_address = spawn_app().await;

        let client = Client::new();
        let response = client
            .get(format!("{}/health", &app_address))
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn subscribe_returns_200_for_valid_form_data() {
        let app_address = spawn_app().await;
        let client = Client::new();
        let body = "name=Test%20User&email=test_user%40test.com";
        let response = client
            .post(format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn subscribe_returns_400_when_missing_data() {
        let app_address = spawn_app().await;
        let client = Client::new();

        let test_cases = vec![
            ("name=Test%20User", "missing email"),
            ("email=test%40test.com", "missing name"),
            ("", "missing both email and name"),
        ];
        for (invalid_body, error_message) in test_cases {
            let response = client
                .post(format!("{}/subscriptions", &app_address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("Failed to execute request.");

            assert_eq!(
                422,
                response.status().as_u16(),
                "The API did not fail with 400 Bad Request when the payload was {}",
                error_message
            );
        }
    }
}
