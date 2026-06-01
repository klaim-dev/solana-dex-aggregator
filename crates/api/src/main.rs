mod app;
mod config;
mod domain;
mod error;
mod infra;
mod routes;
mod state;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::{config::Config, infra::repo::in_memory::InMemoryAccountRepo, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let state = AppState {
        config: Arc::new(config),
        account_repo: Arc::new(InMemoryAccountRepo::new()),
    };
    let app = app(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

pub(crate) fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(routes::health::healthz))
        .route("/readyz", get(routes::health::readyz))
        .route("/status", get(routes::health::status))
        .route("/accounts", post(routes::accounts::create_account))
        .with_state(state)
}
