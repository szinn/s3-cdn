use std::time::Duration;

use axum::{routing::get, Router};
use tower_http::timeout::TimeoutLayer;

use super::HttpFrontend;

pub mod get_object;
pub mod health;

impl HttpFrontend {
    pub fn get_routes(self) -> Router<()> {
        axum::Router::new()
            .route("/health", get(health::health))
            .route("/*path", get(get_object::get_object))
            .with_state(self.document_service.clone())
            .layer(TimeoutLayer::new(Duration::from_secs(2)))
    }
}
