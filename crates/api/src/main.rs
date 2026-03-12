use axum::{routing::get, Router};
use axum::Json;

use crate::error::AppError;
mod error;

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");

    axum::serve(listener, app)
        .await
        .expect("api server crashed");
}

fn create_app() -> Router {
Router::new()
    .route("/healthz", get(healthz))
    .route("/readyz", get(readyz))
    .route("/status", get(status))
   
}

async fn healthz() -> Result<&'static str, AppError> {
    Ok("Ok")
}

async fn readyz() -> Result<&'static str, AppError> {
    Ok("Ok")
}

async fn status() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({ "status": "ok", "version": "0.1.0" })))
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
    let app = create_app();
    
    let response = app
        .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_readyz_returns_200() {
    let app = create_app();
    
    let response = app
        .oneshot(Request::builder().uri("/readyz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_status() {
    let app = create_app();
    
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

}


