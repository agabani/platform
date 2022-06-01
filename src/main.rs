use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health/liveness", get(health_liveness))
        .route("/health/readiness", get(health_readiness));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_liveness() {}

async fn health_readiness() {}
