use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::time::Instant;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::state::AppState;

use std::sync::OnceLock;

static HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

pub fn metrics_handle() -> PrometheusHandle {
    HANDLE
        .get_or_init(|| PrometheusBuilder::new().install_recorder().unwrap())
        .clone()
}

pub async fn record_metrics(request: Request, next: Next) -> Response {
    metrics::counter!("http_requests_total").increment(1);

    let started_at = Instant::now();
    let response = next.run(request).await;
    let latency_secs = started_at.elapsed().as_secs_f64();
    metrics::histogram!("http_request_duration_seconds").record(latency_secs);
    response
}

pub async fn metrics(State(state): State<AppState>) -> impl IntoResponse {
    state.metrics_handle.render()
}
