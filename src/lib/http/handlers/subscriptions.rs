use crate::http::handlers::shared::{ApiError, ApiSuccess};
use axum::http::StatusCode;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Subscribe {
    name: String,
    email: String,
}

pub async fn subscriptions_handler(
    Form(subscribe): Form<Subscribe>,
) -> Result<ApiSuccess<()>, ApiError> {
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
