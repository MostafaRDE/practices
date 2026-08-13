use crate::domain::currency::Currency;
use crate::domain::{pair::Pair, rate::Rate};
use std::collections::HashMap;
use std::sync::{Arc};
use std::time::SystemTime;
use rust_decimal::Decimal;
use tokio::sync::{RwLock};

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

    pub fn replace(&mut self, rates: Vec<Rate>) {
        for rate in rates {
            self.rates.insert(rate.pair, rate);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rates.is_empty()
    }

    pub fn get(&self, pair: &Pair) -> Option<&Rate> {
        self.rates.get(pair)
    }
}

#[test]
fn new_snapshot_shout_be_empty() {
    let snapshot = Snapshot::new();
    assert!(snapshot.is_empty());
}

#[test]
fn replace_should_store_rates() {
    let mut snapshot = Snapshot::new();

    let pair = Pair { base: Currency::USD, quote: Currency::IRR };
    let rate = Rate {
        pair,
        buy: Decimal::new(1700000, 0),
        sell: Decimal::new(1780000, 0),
        updated_at: SystemTime::now(),
    };
    snapshot.replace(vec![rate]);
    assert!(!snapshot.is_empty());

    let result = snapshot.get(&pair);
    assert!(result.is_some());
}

