use std::str::FromStr;

use crate::error::AppError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use validator::Validate;

#[derive(Debug, Deserialize, validator::Validate)]
pub struct CreateAccountRequest {
    #[validate(length(min = 1, max = 64))]
    pub label: String,
    pub pubkey: String,
    pub lamports: u64,
}
#[derive(Debug, Serialize)]
pub struct CreateAccountResponse {
    pub label: String,
    pub pubkey: String,
    pub lamports: u64,
}

pub async fn create_account(
    Json(payload): Json<CreateAccountRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Pubkey::from_str(&payload.pubkey).map_err(|_| AppError::BadRequest("invalid pubkey".into()))?;
    let response = CreateAccountResponse {
        label: payload.label,
        pubkey: payload.pubkey,
        lamports: payload.lamports,
    };
    Ok((StatusCode::CREATED, Json(response)))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use validator::Validate;

    use crate::{app, routes::accounts::CreateAccountRequest};

    #[test]
    fn validate_negative() {
        let payload = CreateAccountRequest {
            label: "a".repeat(100),
            pubkey: "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes".to_string(),
            lamports: 324,
        };

        let res = payload.validate();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn missing_field_returns_422() {
        let request = Request::builder()
            .method("POST")
            .uri("/accounts")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"pubkey":"6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes","lamports":300}"#,
            ))
            .unwrap();

        let response = app().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY)
    }

    #[tokio::test]
    async fn valid_request_returns_201() {
        let request = Request::builder()
        .method("POST")
        .uri("/accounts")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"label":"sol", "pubkey":"6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes","lamports":300}"#))
        .unwrap();

        let response = app().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED)
    }

    #[tokio::test]
    async fn label_too_long_returns_400() {
        let request = Request::builder()
        .method("POST")
        .uri("/accounts")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"label":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","pubkey":"6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes","lamports":300}"#))
        .unwrap();

        let response = app().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST)
    }

    #[tokio::test]
    async fn invalid_pubkey_returns_400() {
        let request = Request::builder()
        .method("POST")
        .uri("/accounts")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"label":"sol","pubkey":"6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfz111","lamports":300}"#))
        .unwrap();

        let response = app().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST)
    }
}
