use crate::domain::{
    rate::Rate,
    provider::Provider,
};

pub struct TgjuProvider {
    request: reqwest::Client,
}

impl Provider for TgjuProvider {
    async fn fetch(&self) -> std::io::Result<Vec<Rate>> {
        todo!()
    }
}
