mod app;
mod config;
mod domain;
mod error;
mod infra;
mod state;
use axum::{
    http::{header::HeaderName, Request},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use crate::{config::Config, infra::repo::in_memory::InMemoryAccountRepo, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("api=info,tower_http=info")),
        )
        .init();

    let config = Config::from_env()?;
    let state = AppState {
        config: Arc::new(config),
        account_repo: Arc::new(InMemoryAccountRepo::new()),
    };
    let app = app(state);
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "listening");
    axum::serve(listener, app).await?;
    Ok(())
}

pub(crate) fn app(state: AppState) -> Router {
    let request_id_header = HeaderName::from_static("x-request-id");

    Router::new()
        .route("/healthz", get(infra::http::health::healthz))
        .route("/readyz", get(infra::http::health::readyz))
        .route("/status", get(infra::http::health::status))
        .route(
            "/accounts",
            post(infra::http::accounts::create_account).get(infra::http::accounts::list_account),
        )
        .route(
            "/accounts/{pubkey}",
            get(infra::http::accounts::get_account),
        )
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    request_id_header.clone(),
                    MakeRequestUuid::default(),
                ))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let request_id = request
                                .extensions()
                                .get::<RequestId>()
                                .and_then(|request_id| request_id.header_value().to_str().ok())
                                .unwrap_or_default();

                            tracing::info_span!(
                                "http_request",
                                request_id,
                                method = %request.method(),
                                uri = %request.uri(),
                                version = ?request.version(),
                            )
                        })
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(PropagateRequestIdLayer::new(request_id_header)),
        )
        .with_state(state)
}
