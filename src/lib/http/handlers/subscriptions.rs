use crate::http::handlers::shared::{ApiError, ApiSuccess};
use anyhow::anyhow;
use axum::Form;
use axum::{extract::State, http::StatusCode};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Subscribe {
    name: String,
    email: String,
}

pub async fn subscriptions_handler(
    Form(form): Form<Subscribe>,
    State(pool): State<PgPool>,
) -> Result<ApiSuccess<()>, ApiError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| {
            anyhow!(e)
                .context(format!("failed to save subscription"))
                .into()
        })
        .unwrap();

    let query = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    );

    tx.execute(query).await?;
    Ok(ApiSuccess::new(StatusCode::OK, ()))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test(flavor = "multi_thread")]
//     async fn test_health_handler() {
//         let response = health_handler().await;
//         assert!(response.is_ok());
//     }
// }
