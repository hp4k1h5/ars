use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use std::time::Instant;

pub async fn log_requests(req: Request, next: Next) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query().map(|q| q.to_string());

    async move {
        let start = Instant::now();

        // Process the request
        let response = next.run(req).await;

        let duration = start.elapsed();
        let status_code = response.status();

        // Log with structured data
        tracing::info!(
            http.method = %method,
            http.path = %path,
            http.query = query.as_deref().unwrap_or(""),
            http.status_code = status_code.as_u16(),
            http.duration_ms = duration.as_millis() as u64,
            "request completed"
        );

        Ok(response)
    }
    .await
}
