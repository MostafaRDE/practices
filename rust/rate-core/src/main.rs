use crate::{providers::mock::MockProvider, services::updater::Updater, state::snapshot::{SharedSnapshot, Snapshot}};

mod domain;
mod providers;
mod services;
mod state;

use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let snapshot: SharedSnapshot = Arc::new(RwLock::new(Snapshot::new()));

    let mut updater = Updater::new(
        MockProvider {},
        snapshot.clone(),
    );

    tokio::spawn(async move {
        updater.run().await;
    });
}
