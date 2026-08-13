use core::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Currency {
    BTC,
    EUR,
    IRR,
    USD,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::BTC => write!(f, "BTC"),
            Currency::EUR => write!(f, "EUR"),
            Currency::IRR => write!(f, "IRR"),
            Currency::USD => write!(f, "USD"),
        }
    }
}
