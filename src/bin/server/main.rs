use email_newsletter::http::HttpServer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let http_server = HttpServer::new().await?;
    http_server.run(listener).await
}
