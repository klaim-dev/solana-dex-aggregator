use axum::{response::IntoResponse, Json};
use serde::Serialize;

use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    name: String,
    version: String,
}

pub async fn healthz() -> Result<impl IntoResponse, AppError> {
    Ok("ok")
}

pub async fn readyz() -> Result<impl IntoResponse, AppError> {
    Ok("ready")
}

pub async fn status() -> Result<Json<StatusResponse>, AppError> {
    let response = StatusResponse {
        name: "api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(Json(response))
}

#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::app;
    use axum::body::to_bytes;

    use super::*;

    #[tokio::test]
    async fn healthz_returns_200() {
        let app = app();
        let request = Request::builder()
            .uri("/healthz")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body_str.contains("ok"));
    }

    #[test]
    fn app_error_not_found_maps_to_404() {
        let response = AppError::NotFound.into_response();
        let status = response.status();
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]

    async fn app_error_bad_request_includes_message() {
        let response = AppError::BadRequest("amount must be positive".into()).into_response();
        let status = response.status();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body_str.contains("amount must be positive"));
    }
}
