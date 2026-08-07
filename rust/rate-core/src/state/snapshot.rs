use crate::domain::{pair::Pair, rate::Rate};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Snapshot {
    rates: Arc<RwLock<HashMap<Pair, Rate>>>,
}

impl Snapshot {
    pub fn new() -> Self {
        Self {
            rates: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn replace(&self, rates: Vec<Rate>) {}
}
