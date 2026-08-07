use super::currency::{ Currency };

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Pair {
    pub base: Currency,
    pub quote: Currency,
}
