use crate::{domain::provider::{Provider, ProviderError}, state::snapshot::SharedSnapshot};

pub struct Updater<P: Provider> {
    pub provider: P,
    snapshot: SharedSnapshot,
}

impl<P: Provider> Updater<P> {
    pub fn new(provider: P, snapshot: SharedSnapshot) -> Self {
        Self {
            provider,
            snapshot,
        }
    }

    pub async fn run(&self) {
        loop {
            self.update_once().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn update_once(&self) -> std::result::Result<(), ProviderError> {
        let data = self.provider.fetch().await?;
        let mut snapshot = self.snapshot.write().await;
        snapshot.upsert(data);
        Ok(())
        // match self.provider.fetch().await {
        //     Ok(data) => {
        //         let mut snapshot = self.snapshot.write().await;
        //         snapshot.upsert(data)
        //     },
        //     Err(error) => ,
        // };
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::{currency::Currency, pair::Pair, rate::Rate}, providers::mock::MockProvider};
    use crate::state::snapshot::Snapshot;
    use rust_decimal::Decimal;
    use std::sync::Arc;
    use std::time::SystemTime;
    use super::*;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn should_update_snapshot_from_provider() {
        let pair = Pair {base: Currency::USD, quote: Currency::IRR};
        // let rate = Rate {
        //     pair,
        //     buy: Decimal::new(170_000_0, 0),
        //     sell: Decimal::new(178_000_0, 0),
        //     updated_at: SystemTime::now(),
        // };
        let provider = MockProvider {};
        let snapshot: SharedSnapshot = Arc::new(RwLock::new(Snapshot::new()));
        let updater = Updater::new(provider, snapshot.clone());
        updater.update_once().await.unwrap();

        let snapshot = snapshot.read().await;
        let result = snapshot.get(&pair);
        assert!(result.is_some());
    }
}
