use crate::{services::updater::Updater, state::snapshot::Snapshot};

mod domain;
mod providers;
mod services;
mod state;

#[tokio::main]
async fn main() {
    let updater = Updater::new();

    let snapshot = Snapshot::new();

    let updater_snapshot = snapshot.clone();

    tokio::spawn(async move {
        updater.run(updater_snapshot).await;
    });
}
