use crate::domain::{pair::Pair, rate::Rate};
use std::collections::HashMap;
use std::sync::{Arc};
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
    pub fn replace(&self, rates: Vec<Rate>) {}
}
