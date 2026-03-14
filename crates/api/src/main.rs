use axum::extract::State;
use axum::routing::post;
use axum::{routing::get, Router};
use axum::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;
use dotenvy::{self, dotenv};
use std::sync::Arc;

use crate::error::AppError;
use crate::state::{AppState, Config};
mod error;
mod state;

#[tokio::main]
async fn main() {
   dotenv().ok();

    let config = Config::from_env().unwrap();

    let app = create_app(config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");

    axum::serve(listener, app)
        .await
        .expect("api server crashed");

}


fn create_app(config: Config) -> Router {
    let state = Arc::new(AppState{config});
Router::new()
    .route("/healthz", get(healthz))
    .route("/readyz", get(readyz))
    .route("/status", get(status))
    .route("/account", post(account))
    .with_state(state)

}

async fn healthz() -> Result<&'static str, AppError> {
    Ok("Ok")
}

async fn readyz() -> Result<&'static str, AppError> {
    Ok("Ok")
}

async fn status(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, AppError> {
let solana_rpc_client = state.config.get_solana_rpc_client();
    Ok(Json(serde_json::json!({ "status": "ok", "version": "0.1.0", "solana_rpc_client": solana_rpc_client })))
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct AccountRequest {
    #[validate(length(max = 44))]
    pubkey: String,
    #[validate(length(max = 32))]
    label: String,
    #[validate(range(min = 0, max = 1000000))]
    amount: u64,
}

async fn account(Json(input): Json<AccountRequest>) -> Result<Json<AccountRequest>, AppError> {
    input.validate().map_err(|_| AppError::BadRequest { input: "invalid input".to_string() })?;
    Ok(Json(input))
}


#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;
    use axum::http::Request;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use axum::http::StatusCode;

    
#[tokio::test]
async fn test_healthz_returns_200() {
    let config = Config::from_env().unwrap();
    let app = create_app(config);
    
    let response = app
        .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_readyz_returns_200() {
    let config = Config::from_env().unwrap();
    let app = create_app(config);

    let response = app
        .oneshot(Request::builder().uri("/readyz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_status() {
    let config = Config::from_env().unwrap();
    let app = create_app(config);

    let response = app
        .oneshot(Request::builder().uri("/status").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let (parts, body) = response.into_parts();
     let body = body.collect().await.unwrap().to_bytes();
     let json: serde_json::Value = serde_json::from_slice(&body).unwrap(); 

    assert_eq!(parts.status, StatusCode::OK);
    assert_eq!(json["status"], "ok");
    assert_eq!(json["version"], "0.1.0");
}

#[tokio::test]
async fn test_account_returns_400() {
    let config = Config::from_env().unwrap();
    let app = create_app(config);

    let payload = serde_json::json!({
    "pubkey": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "label": "test",
    "amount": 100,
});
    
    let response = app
    .oneshot(Request::builder()
    .method("POST")
    .uri("/account")
    .header("content-type", "application/json")
    .body(Body::from(serde_json::to_string(&payload).unwrap()))
    .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_account_returns_400_lable_lenght_than_32() {
    let config = Config::from_env().unwrap();
    let app = create_app(config);

    let payload = serde_json::json!({
    "pubkey": "aaaa",
    "label": "testtesttesttesttesttesttesttesttesttesttesttesttest",
    "amount": 100,
});
    
    let response = app
    .oneshot(Request::builder()
    .method("POST")
    .uri("/account")
    .header("content-type", "application/json")
    .body(Body::from(serde_json::to_string(&payload).unwrap()))
    .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_account_returns_400_lable_amount_invalid_range() {
    let config = Config::from_env().unwrap();
    let app = create_app(config);

    let payload = serde_json::json!({
    "pubkey": "aaaa",
    "label": "test",
    "amount": 2000000,
});
    
    let response = app
    .oneshot(Request::builder()
    .method("POST")
    .uri("/account")
    .header("content-type", "application/json")
    .body(Body::from(serde_json::to_string(&payload).unwrap()))
    .unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}


}


