use anyhow::Context;
use email_newsletter::configuration::get_configuration;
use email_newsletter::http::HttpServer;
use sqlx::{Connection, PgPool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_configuration().expect("Failed to read configuration.");
    let connection_string = config.database.connection_string();

    let pool = PgPool::connect_with(
        PgConnectOptions::from_str(connection_string)
            .with_context(|| format!("invalid database path: {}", path))?,
    )
    .await
    .with_context(|| format!("failed to open database at: {}", path))?;

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).await.unwrap();

    let http_server = HttpServer::new(pool).await?;
    http_server.run(listener).await
}
