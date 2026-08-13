use super::handlers;
use crate::state::snapshot::SharedSnapshot;
use axum::{Router, routing::get};

pub fn router(snapshot: SharedSnapshot) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/rates", get(handlers::get_rates))
        .with_state(snapshot)
}
