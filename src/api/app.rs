use axum::response::Json;

#[derive(Clone)]
pub struct AppState {}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

/// Service root
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Service name", body = str)
    ),
    tag = "meta"
)]
pub async fn root() -> &'static str {
    "ars.wiki - Language API"
}

/// Health check
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service health", body = HealthResponse)
    ),
    tag = "meta"
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "ars.wiki".to_string(),
    })
}
