use crate::domain::{pair::Pair, rate::Rate};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Snapshot {
    rates: Arc<RwLock<HashMap<Pair, Rate>>>,
}
