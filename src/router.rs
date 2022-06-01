use axum::{
    routing::{get, post},
    Router,
};

use crate::handler;

pub fn create() -> Router {
    Router::new()
        .route("/account/signup", post(handler::accounts::signup))
        .route("/health/liveness", get(handler::health::liveness))
        .route("/health/readiness", get(handler::health::readiness))
}
