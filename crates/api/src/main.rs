mod app;
mod config;
mod domain;
mod error;
mod infra;
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
        .route("/healthz", get(infra::http::health::healthz))
        .route("/readyz", get(infra::http::health::readyz))
        .route("/status", get(infra::http::health::status))
        .route("/accounts", post(infra::http::accounts::create_account))
        .route("/accounts/{pubkey}", get(infra::http::accounts::get_account))
        .with_state(state)
}
