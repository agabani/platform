use axum::{routing::get, Router};

pub fn create() -> Router {
    Router::new()
        .route("/health/liveness", get(health_liveness))
        .route("/health/readiness", get(health_readiness))
}

async fn health_liveness() {}

async fn health_readiness() {}
