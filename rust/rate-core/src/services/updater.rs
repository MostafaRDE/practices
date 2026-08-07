use crate::{domain::provider::Provider, providers::mock::MockProvider, state::snapshot::Snapshot};

pub struct Updater<P: Provider> {
    pub provider: P,
}

impl Updater<MockProvider> {
    pub fn new() -> Self {
        Self {
            provider: MockProvider {},
        }
    }
    pub async fn run(&self, snapshot: Snapshot) {
        match self.provider.fetch().await {
            Ok(data) => snapshot.replace(data),
            Err(_) => todo!(),
        };
    }
}
