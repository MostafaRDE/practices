use crate::domain::{
    currency::Currency, pair::Pair, provider::{Provider, ProviderError}, rate::Rate,
};
use rust_decimal::Decimal;
use std::time::SystemTime;

pub struct MockProvider {}

impl Provider for MockProvider {
    async fn fetch(&self) -> std::result::Result<Vec<Rate>, ProviderError> {
        let buy_eur = Decimal::new(rand::random_range(202_000_0..228_000_0), 0);
        let buy_usd = Decimal::new(rand::random_range(160_000_0..200_000_0), 0);
        Ok(vec![
            Rate {
                pair: Pair { base: Currency::EUR, quote: Currency::IRR },
                buy: buy_eur,
                sell: buy_eur * Decimal::new(105, 2).floor(),
                updated_at: SystemTime::now(),
            },
            Rate {
                pair: Pair { base: Currency::USD, quote: Currency::IRR },
                buy: buy_usd,
                sell: buy_usd * Decimal::new(105, 2).floor(),
                updated_at: SystemTime::now(),
            },
        ])
    }
}
