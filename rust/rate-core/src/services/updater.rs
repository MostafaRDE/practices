use crate::{domain::provider::Provider, state::snapshot::SharedSnapshot};

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

    pub async fn run(&mut self) {
        loop {
            self.update_once().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn update_once(&mut self) {
        match self.provider.fetch().await {
            Ok(data) => {
                let snapshot = &mut self.snapshot.write().await;
                snapshot.replace(data)
            },
            Err(_) => todo!(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_update_snapshot() {}
}
