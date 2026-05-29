mod error;
mod routes;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/healthz", get(routes::health::healthz))
        .route("/readyz", get(routes::health::readyz))
        .route("/status", get(routes::health::status))
}
