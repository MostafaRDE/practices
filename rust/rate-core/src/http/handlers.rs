use crate::state::snapshot::SharedSnapshot;
use axum::{Json, extract::State};
use serde::Serialize;

pub async fn health() -> &'static str {
    "OK"
}


#[derive(Serialize)]
pub struct RatesResponse {
    pub rates: Vec<RateResponse>,
}

#[derive(Serialize)]
pub struct RateResponse {
    pub base: String,
    pub quote: String,
    pub buy: String,
    pub sell: String,
}

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
