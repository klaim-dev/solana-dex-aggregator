use metrics_exporter_prometheus::PrometheusHandle;
use sqlx::PgPool;

use crate::config::Config;
use crate::domain::AccountRepo;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub account_repo: Arc<dyn AccountRepo>,
    pub started_at: Instant,
    pub metrics_handle: PrometheusHandle,
    pub pool: PgPool,
}
