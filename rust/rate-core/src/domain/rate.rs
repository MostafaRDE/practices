use crate::domain::pair::Pair;
use rust_decimal::Decimal;
use std::time::SystemTime;

#[derive(Clone, Copy)]
pub struct Rate {
    pub pair: Pair,
    pub buy: Decimal,
    pub sell: Decimal,
    pub updated_at: SystemTime,    
}
