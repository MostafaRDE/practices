use crate::domain::{
    provider::{Provider, ProviderError}, rate::Rate,
};

pub struct MockProvider {}

impl Provider for MockProvider {
    async fn fetch(&self) -> std::result::Result<Vec<Rate>, ProviderError> {
        todo!()
    }
}
