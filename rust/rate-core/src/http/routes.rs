use super::handlers;
use crate::{http::ApiDoc, state::snapshot::SharedSnapshot};
use axum::{Router, routing::get};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn router(snapshot: SharedSnapshot) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/rates", get(handlers::get_rates))
        .merge(SwaggerUi::new("/docs/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(snapshot)
}
