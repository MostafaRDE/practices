use crate::state::snapshot::SharedSnapshot;
use axum::{Json, extract::State};
use serde::Serialize;

#[utoipa::path(
    get,
    path = "/health",
    tag = "rates",
    responses(
        (status = 200, description = "Service is healthy"),
    ),
)]
pub async fn health() -> &'static str {
    "OK"
}


#[derive(Serialize, utoipa::ToSchema)]
pub struct RatesResponse {
    pub rates: Vec<RateResponse>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct RateResponse {
    pub base: String,
    pub quote: String,
    pub buy: String,
    pub sell: String,
}

#[utoipa::path(
    get,
    path = "/rates",
    tag = "rates",
    responses(
        (
            status = 200,
            description = "Current currency rates",
            body = RatesResponse
        )
    )
)]
pub async fn get_rates(
    State(snapshot): State<SharedSnapshot>,
) -> Json<RatesResponse> {
    let rates = {
        let snapshot = snapshot.read().await;
        snapshot.all()
    };

    let rates = rates
        .into_iter()
        .map(|rate| RateResponse {
            base: rate.pair.base.to_string(),
            quote: rate.pair.quote.to_string(),
            buy: rate.buy.to_string(),
            sell: rate.sell.to_string(),
        })
        .collect();

    Json(RatesResponse { rates })
}
