pub mod app;
pub mod config;
pub mod domain;
pub mod error;
pub mod infra;
pub mod state;

use axum::{
    http::{header::HeaderName, Request},
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub use app::AccountService;
pub use state::AppState;

pub fn app(state: AppState) -> Router {
    let request_id_header = HeaderName::from_static("x-request-id");

    Router::new()
        .route("/healthz", get(infra::http::health::healthz))
        .route("/readyz", get(infra::http::health::readyz))
        .route("/status", get(infra::http::health::status))
        .route("/metrics", get(infra::http::metrics::metrics))
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
                .layer(axum::middleware::from_fn(
                    infra::http::metrics::record_metrics,
                ))
                .layer(PropagateRequestIdLayer::new(request_id_header)),
        )
        .with_state(state)
}
