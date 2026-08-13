use crate::{providers::mock::MockProvider, services::updater::Updater, state::snapshot::{SharedSnapshot, Snapshot}};

mod domain;
mod http;
mod providers;
mod services;
mod state;

use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let snapshot: SharedSnapshot = Arc::new(RwLock::new(Snapshot::new()));

    let updater = Updater::new(
        MockProvider {},
        snapshot.clone(),
    );

    tokio::spawn(async move {
        updater.run().await;
    });

    let app = http::routes::router();
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("App run on: http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
