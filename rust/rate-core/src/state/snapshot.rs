use crate::domain::{pair::Pair, rate::Rate};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Snapshot {
    rates: HashMap<Pair, Rate>,
}

pub type SharedSnapshot = Arc<RwLock<Snapshot>>;

impl Snapshot {
    pub fn new() -> Self {
        Self {
            rates: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rates.is_empty()
    }

    pub fn get(&self, pair: &Pair) -> Option<&Rate> {
        self.rates.get(pair)
    }

    pub fn upsert(&mut self, rates: Vec<Rate>) {
        for rate in rates {
            self.rates.insert(rate.pair, rate);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::currency::Currency;
    use rust_decimal::Decimal;
    use std::time::SystemTime;
    use super::*;

    #[test]
    fn new_snapshot_shout_be_empty() {
        let snapshot = Snapshot::new();
        assert!(snapshot.is_empty());
    }

    #[test]
    fn upsert_should_store_rates() {
        let mut snapshot = Snapshot::new();

        let pair = Pair {
            base: Currency::USD,
            quote: Currency::IRR,
        };
        let rate = Rate {
            pair,
            buy: Decimal::new(1700000, 0),
            sell: Decimal::new(1780000, 0),
            updated_at: SystemTime::now(),
        };
        snapshot.upsert(vec![rate]);
        assert!(!snapshot.is_empty());

        let result = snapshot.get(&pair);
        assert!(result.is_some());
    }

    #[test]
    fn upsert_should_store_rate_values() {
        let mut snapshot = Snapshot::new();

        let pair = Pair {
            base: Currency::USD,
            quote: Currency::IRR,
        };
        let rate = Rate {
            pair,
            buy: Decimal::new(1700000, 0),
            sell: Decimal::new(1780000, 0),
            updated_at: SystemTime::now(),
        };
        snapshot.upsert(vec![rate]);

        let result = snapshot.get(&pair).unwrap();
        assert_eq!(result.buy, Decimal::new(1700000, 0));
        assert_eq!(result.sell, Decimal::new(1780000, 0));
    }

    #[test]
    fn get_should_return_existing_rate() {
        let mut snapshot = Snapshot::new();

        let pair = Pair { base: Currency::USD, quote: Currency::IRR };
        snapshot.upsert(vec![ Rate {
            pair,
            buy: Decimal::new(170_000_0, 0),
            sell: Decimal::new(178_000_0, 0),
            updated_at: SystemTime::now(),
        } ]);

        let result = snapshot.get(&pair);
        assert!(result.is_some());
    }

    #[test]
    fn get_should_return_none_for_unknown_pair() {
        let snapshot = Snapshot::new();
        let result = snapshot.get(&Pair { base: Currency::USD, quote: Currency::IRR });
        assert!(result.is_none());
    }

    #[test]
    fn upsert_should_not_remove_old_rates() {
        let mut snapshot = Snapshot::new();

        let usd_irr_pair = Pair { base: Currency::USD, quote: Currency::IRR };
        let eur_irr_pair = Pair { base: Currency::EUR, quote: Currency::IRR };
        let btc_irr_pair = Pair { base: Currency::BTC, quote: Currency::IRR };

        let usd_irr_rate = Rate {
            pair: usd_irr_pair,
            buy: Decimal::new(170_000_0, 0),
            sell: Decimal::new(178_000_0, 0),
            updated_at: SystemTime::now(),
        };
        let eur_irr_rate = Rate {
            pair: eur_irr_pair,
            buy: Decimal::new(210_000_0, 0),
            sell: Decimal::new(232_000_0, 0),
            updated_at: SystemTime::now(),
        };
        let btc_irr_rate = Rate {
            pair: btc_irr_pair,
            buy: Decimal::new(1_678_000_000_0, 0),
            sell: Decimal::new(1_801_000_000_0, 0),
            updated_at: SystemTime::now(),
        };

        snapshot.upsert(vec![usd_irr_rate, eur_irr_rate]);
        snapshot.upsert(vec![usd_irr_rate, btc_irr_rate]);

        assert!(snapshot.get(&usd_irr_pair).is_some());
        assert!(snapshot.get(&eur_irr_pair).is_some());
        assert!(snapshot.get(&btc_irr_pair).is_some());
    }
}
