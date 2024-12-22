use axum::http::StatusCode;

pub async fn health_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_health_handler() {
        let (status, body) = health_handler().await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "OK");
    }
}
