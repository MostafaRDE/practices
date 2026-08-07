use crate::domain::{
    currency::Currency, pair::Pair, provider::{Provider, ProviderError}, rate::Rate,
};
use rust_decimal::Decimal;
use std::time::SystemTime;

pub struct MockProvider {}

impl Provider for MockProvider {
    async fn fetch(&self) -> std::result::Result<Vec<Rate>, ProviderError> {
        let buy = Decimal::new(rand::random_range(1_600_000..2_000_000), 0);
        Ok(vec![
            Rate {
                pair: Pair { base: Currency::USD, quote: Currency::IRR },
                buy,
                sell: buy * Decimal::new(105, 2).floor(),
                updated_at: SystemTime::now(),
            }
        ])
    }
}
