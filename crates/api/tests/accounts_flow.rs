use api::{
    app,
    config::Config,
    infra::{http::metrics::metrics_handle, repo::in_memory::InMemoryAccountRepo},
    AppState,
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use solana_sdk::pubkey::Pubkey;
use sqlx::postgres::PgPoolOptions;
use std::{sync::Arc, time::Instant};
use tower::ServiceExt;

fn test_state() -> AppState {
    let config = Config {
        solana_rpc_url: "http://localhost:8899".to_string(),
        database_url: "postgres://test".to_string(),
        jwt_secret: "test-secret".to_string(),
    };

    AppState {
        config: Arc::new(config),
        account_repo: Arc::new(InMemoryAccountRepo::new()),
        started_at: Instant::now(),
        metrics_handle: metrics_handle(),
        pool: PgPoolOptions::new()
            .connect_lazy("postgres://test:test@localhost/test")
            .unwrap(),
    }
}

#[tokio::test]
async fn create_account() {
    let state = test_state();
    let app = app(state);
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"pubkey":"6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes","owner":"11111111111111111111111111111111","lamports":300}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn get_account() {
    let state = test_state();
    let app = app(state);
    let pubkey = "6HTpFxctmd8qm5a5gxjHztsnfKyMJQxmafLCgzpLfzes";

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"pubkey":"{pubkey}","owner":"11111111111111111111111111111111","lamports":300}}"#
                )))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_response.status(), StatusCode::CREATED);

    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/accounts/{pubkey}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);
    let body = to_bytes(get_response.into_body(), usize::MAX)
        .await
        .unwrap();

    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["pubkey"], pubkey);
}

#[tokio::test]
async fn create_get_list_flow() {
    let state = test_state();
    let app = app(state);

    let mut pubkeys = Vec::new();

    for _ in 0..3 {
        let pubkey = Pubkey::new_unique();
        let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"pubkey":"{pubkey}","owner":"11111111111111111111111111111111","lamports":300}}"#
                )))
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(create_response.status(), StatusCode::CREATED);
        pubkeys.push(pubkey);
    }

    for pubkey in &pubkeys {
        let get_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/accounts/{pubkey}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(get_response.status(), StatusCode::OK)
    }

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/accounts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(list_response.status(), StatusCode::OK);

    let body = to_bytes(list_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json.as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn status_reports_uptime() {
    let state = test_state();
    let app = app(state);

    let status_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = to_bytes(status_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["uptime_secs"].as_u64().is_some());
    assert!(json.get("jwt_secret").is_none());
}

#[tokio::test]

async fn metrics_endpoint_reports_counts() {
    let state = test_state();
    let app = app(state);

    for _ in 0..2 {
        let list_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/accounts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(list_response.status(), StatusCode::OK);
    }

    let metrics = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = to_bytes(metrics.into_body(), usize::MAX).await.unwrap();
    let text = std::str::from_utf8(&body).unwrap();
    assert!(text.contains("http_requests_total"));
    assert!(text.contains("http_request_duration_seconds"));
}
