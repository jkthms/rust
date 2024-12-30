use std::sync::Arc;

use axum::{
    routing::get,
    Router
};

use crate::{
    handler::{health_checker_handler, fibonacci_handler},
    AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthz", get(health_checker_handler))
        .route("/api/fibonacci", get(fibonacci_handler))
        .with_state(state)
}