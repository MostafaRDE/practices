pub mod handlers;
pub mod routes;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health,
        handlers::get_rates,
    ),
    components(
        schemas(
            handlers::RatesResponse,
            handlers::RateResponse,
        )
    ),
    tags(
        (name = "rates", description = "Currency rate API")
    )
)]
pub struct ApiDoc;
