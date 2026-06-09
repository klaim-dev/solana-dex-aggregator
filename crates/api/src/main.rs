use api::{
    app,
    config::Config,
    infra::{http::metrics::metrics_handle, repo::in_memory::InMemoryAccountRepo},
    AppState,
};
use sqlx::PgPool;
use std::{sync::Arc, time::Instant};
use tracing_subscriber::EnvFilter;

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

    let metrics_handle = metrics_handle();

    let config = Config::from_env()?;
    let pool = PgPool::connect(&config.database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let state = AppState {
        config: Arc::new(config),
        account_repo: Arc::new(InMemoryAccountRepo::new()),
        started_at: Instant::now(),
        metrics_handle,
        pool,
    };
    let app = app(state);
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "listening");
    axum::serve(listener, app).await?;
    Ok(())
}
