use axum::response::IntoResponse;
use hyper::StatusCode;

#[tracing::instrument(level = "trace")]
pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}
