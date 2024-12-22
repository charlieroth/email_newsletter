use crate::http::handlers::shared::{ApiError, ApiSuccess};
use axum::http::StatusCode;

pub async fn health_handler() -> Result<ApiSuccess<&'static str>, ApiError> {
    Ok(ApiSuccess::new(StatusCode::OK, "OK"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_health_handler() {
        let response = health_handler().await;
        assert!(response.is_ok());
    }
}
