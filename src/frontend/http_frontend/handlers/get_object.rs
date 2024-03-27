use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::HeaderValue;
use axum::response::{IntoResponse, Response};
use hyper::{header, StatusCode};
use mime_guess::MimeGuess;
use serde::{Deserialize, Serialize};

use crate::core::document_service::DocumentService;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    path: String,
}

#[tracing::instrument(level = "debug", skip(document_service))]
pub async fn get_object(State(document_service): State<Arc<dyn DocumentService>>, Path(params): Path<Params>) -> Response {
    let result = document_service.get_object(&params.path).await;
    match result {
        Ok(contents) => {
            let is_ascii = contents.is_ascii();
            let mut res = Body::from(contents).into_response();
            let content_type = determine_content_type(&params.path, is_ascii);
            res.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_str(&content_type).unwrap());
            res
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn determine_content_type(path: &str, is_ascii: bool) -> String {
    let mime_guess = MimeGuess::from_path(path);
    tracing::info!("Path: {:?} is mime type: {:?}", path, mime_guess);

    if mime_guess.is_empty() {
        if is_ascii {
            "text/plain".to_string()
        } else {
            "image/png".to_string() // "application/octet-stream".to_string()
        }
    } else {
        mime_guess.first().unwrap().to_string()
    }
}
