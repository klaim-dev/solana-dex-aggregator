use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;

use crate::{error::AppError, state::AppState};

pub async fn healthz() -> Result<impl IntoResponse, AppError> {
    Ok("ok")
}

pub async fn readyz() -> Result<impl IntoResponse, AppError> {
    Ok("ready")
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub solana_rpc_url: String,
    pub uptime_secs: u64,
}

pub async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    let solana_rpc_url = state.config.solana_rpc_url.clone();
    let uptime_secs = state.started_at.elapsed().as_secs();
    let response = StatusResponse {
        solana_rpc_url,
        uptime_secs,
    };
    Json(response)
}

#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use axum::body::to_bytes;
    use std::{sync::Arc, time::Instant};

    use crate::{
        app, config::Config,
        infra::{http::metrics::metrics_handle, repo::in_memory::InMemoryAccountRepo},
    };

    use super::*;

    fn test_state() -> crate::state::AppState {
        crate::state::AppState {
            config: Arc::new(Config {
                solana_rpc_url: "a".to_string(),
                database_url: "a".to_string(),
                jwt_secret: "a".to_string(),
            }),
            account_repo: Arc::new(InMemoryAccountRepo::new()),
            started_at: Instant::now(),
            metrics_handle: metrics_handle(),
        }
    }

    #[tokio::test]
    async fn healthz_returns_200() {
        let app = app(test_state());
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
