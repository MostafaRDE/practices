use std::sync::{Arc, RwLock};
use tokio::task::JoinHandle;

pub async fn run() {
    let storage: Arc<RwLock<i32>> = Arc::new(RwLock::new(0));

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..10 {
        let shared = Arc::clone(&storage);
        handles.push(tokio::spawn(async move {
            // let writer = shared.write();
            let value = shared.read().unwrap();
            // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("Data of {} index is: {}", i, value);
        }));
    }

    for handle in handles {
        match handle.await {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    let final_value = storage.read().unwrap();
    println!("Final value: {}", final_value);
}
