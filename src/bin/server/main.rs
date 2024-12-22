use email_newsletter::http::HttpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_server = HttpServer::new().await?;
    http_server.run().await
}
